use std::collections::HashMap;

use crossbeam_channel::{Receiver, Sender};
use wg_config::Config;
use wg_network::NodeId;

use crate::Command;





pub struct SimControllerOptions {
    pub command_send: HashMap<NodeId, Sender<Command>>,
    pub packet_send: HashMap<NodeId, Sender<Packet>>,
    pub command_recv: Receiver<Command>,
    pub config: Config,
}

pub trait SimulationController {
    fn new(opt: SimControllerOptions) -> Self;
    fn run(&mut self);
}
