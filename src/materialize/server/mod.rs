// Copyright 2019 Materialize, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Materialize, Inc.

//! Main materialized server.

use futures::Future;
use log::error;
use std::boxed::Box;
use std::error::Error as StdError;
use std::net::SocketAddr;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use crate::dataflow;
use crate::glue::*;
use crate::pgwire;
use crate::queue;
use ore::collections::CollectionExt;
use ore::future::FutureExt;
use ore::netio;
use ore::netio::SniffingStream;

mod http;

pub enum QueueConfig {
    Transient,
}

/// Options for how to return dataflow results.
pub enum DataflowResultsConfig {
    /// A process local exchange fabric.
    Local,
    /// An address for an HTTP listener.
    Remote(String),
}

pub struct Config {
    queue: QueueConfig,
    dataflow_results: DataflowResultsConfig,
    timely_configuration: timely::Configuration,
}

impl Config {
    /// Constructs a materialize configuration from a timely dataflow configuration.
    pub fn from_timely(timely_configuration: timely::Configuration) -> Self {
        let post_address = match &timely_configuration {
            timely::Configuration::Thread => {
                "http://localhost:6875/api/dataflow-results".to_owned()
            }
            timely::Configuration::Process(_) => {
                "http://localhost:6875/api/dataflow-results".to_owned()
            }
            timely::Configuration::Cluster { addresses, .. } => {
                let address = addresses[0]
                    .split(':')
                    .next()
                    .expect("Failed to find port in timely address");
                format!("http://{}:6875/api/dataflow-results", address)
            }
        };

        Self {
            queue: QueueConfig::Transient,
            dataflow_results: DataflowResultsConfig::Remote(post_address),
            timely_configuration,
        }
    }

    /// The number of timely workers described the by the configuration.
    pub fn num_timely_workers(&self) -> usize {
        match &self.timely_configuration {
            timely::Configuration::Thread => 1,
            timely::Configuration::Process(n) => *n,
            timely::Configuration::Cluster {
                threads, addresses, ..
            } => threads * addresses.len(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_timely(timely::Configuration::Thread)
    }
}

fn handle_connection(
    tcp_stream: TcpStream,
    sql_command_sender: UnboundedSender<(SqlCommand, CommandMeta)>,
    sql_response_mux: SqlResponseMux,
    dataflow_results_mux: DataflowResultsMux,
    num_timely_workers: usize,
) -> impl Future<Item = (), Error = ()> {
    // Sniff out what protocol we've received. Choosing how many bytes to sniff
    // is a delicate business. Read too many bytes and you'll stall out
    // protocols with small handshakes, like pgwire. Read too few bytes and
    // you won't be able to tell what protocol you have. For now, eight bytes
    // is the magic number, but this may need to change if we learn to speak
    // new protocols.
    let ss = SniffingStream::new(tcp_stream);
    netio::read_exact_or_eof(ss, [0; 8])
        .from_err()
        .and_then(move |(ss, buf, nread)| {
            let buf = &buf[..nread];
            if pgwire::match_handshake(buf) {
                pgwire::serve(
                    ss.into_sniffed(),
                    sql_command_sender,
                    sql_response_mux,
                    dataflow_results_mux,
                    num_timely_workers,
                )
                .either_a()
            } else if http::match_handshake(buf) {
                http::handle_connection(ss.into_sniffed(), dataflow_results_mux).either_b()
            } else {
                reject_connection(ss.into_sniffed()).from_err().either_c()
            }
        })
        .map_err(|err| error!("error handling request: {}", err))
}

fn reject_connection<A: AsyncWrite>(a: A) -> impl Future<Item = (), Error = io::Error> {
    io::write_all(a, "unknown protocol\n").discard()
}

/// Start the materialized server.
pub fn serve(config: Config) -> Result<LocalInputMux, Box<dyn StdError>> {
    // Construct shared channels for SQL command and result exchange, and dataflow command and result exchange.
    let (sql_command_sender, sql_command_receiver) =
        crate::glue::unbounded::<(SqlCommand, CommandMeta)>();
    let sql_response_mux = SqlResponseMux::default();
    let (dataflow_command_sender, dataflow_command_receiver) =
        crate::glue::unbounded::<(DataflowCommand, CommandMeta)>();
    let dataflow_results_mux = DataflowResultsMux::default();

    // Extract timely dataflow parameters.
    let num_timely_workers = config.num_timely_workers();
    let is_primary = match &config.timely_configuration {
        timely::Configuration::Thread => true,
        timely::Configuration::Process(_) => true,
        timely::Configuration::Cluster { process, .. } => process == &0,
    };

    // Initialize pgwire / http listener.
    let listener = if is_primary {
        let listen_addr: SocketAddr = "0.0.0.0:6875".parse()?;
        let listener = TcpListener::bind(&listen_addr)?;
        println!("materialized listening on {}...", listen_addr);
        Some(listener)
    } else {
        None
    };

    // Construct timely dataflow instance.
    let local_input_mux = LocalInputMux::default();
    let dataflow_results_handler = match config.dataflow_results {
        DataflowResultsConfig::Local => {
            dataflow::DataflowResultsHandler::Local(dataflow_results_mux.clone())
        }
        DataflowResultsConfig::Remote(address) => dataflow::DataflowResultsHandler::Remote(address),
    };
    let dd_workers = dataflow::serve(
        dataflow_command_receiver,
        local_input_mux.clone(),
        dataflow_results_handler,
        config.timely_configuration,
        Some(Default::default()), // 10ms logging granularity
    )?;

    // Initialize command queue and sql planner
    match &config.queue {
        QueueConfig::Transient => {
            let worker0_thread = dd_workers.guards().into_first().thread();
            queue::transient::serve(
                sql_command_receiver,
                sql_response_mux.clone(),
                dataflow_command_sender,
                worker0_thread.clone(),
            );
        }
    }

    // Draw connections off of the listener.
    if let Some(listener) = listener {
        let start = future::lazy(move || {
            let server = listener
                .incoming()
                .for_each(move |stream| {
                    tokio::spawn(handle_connection(
                        stream,
                        sql_command_sender.clone(),
                        sql_response_mux.clone(),
                        dataflow_results_mux.clone(),
                        num_timely_workers,
                    ));
                    Ok(())
                })
                .map_err(|err| error!("error accepting connection: {}", err));
            tokio::spawn(server);

            Ok(())
        });
        tokio::run(start);
    }

    Ok(local_input_mux)
}
