use wg_network::topology::Node;

pub struct SimControllerOptions {
    pub command_send: HashMap<NodeId, Sender<Command>>,
    pub command_recv: Receiver<Command>,
    pub config: Config,
}

pub trait SimulationController {
    fn new(opt: SimControllerOptions) -> Self;
    fn run(&mut self);
}
