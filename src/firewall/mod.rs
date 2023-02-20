use crate::error::{NetavarkResult};
use crate::network::internal_types::{
    PortForwardConfig, SetupNetwork, TearDownNetwork, TeardownPortForward,
};
use log::{debug, info};

pub mod firewalld;
pub mod iptables;
pub mod fwnone;
mod varktables;

/// Firewall drivers have the ability to set up per-network firewall forwarding
/// and port mappings.
pub trait FirewallDriver {
    /// Set up firewall rules for the given network,
    fn setup_network(&self, network_setup: SetupNetwork) -> NetavarkResult<()>;
    /// Tear down firewall rules for the given network.
    fn teardown_network(&self, tear: TearDownNetwork) -> NetavarkResult<()>;

    /// Set up port-forwarding firewall rules for a given container.
    fn setup_port_forward(&self, setup_pw: PortForwardConfig) -> NetavarkResult<()>;
    /// Tear down port-forwarding firewall rules for a single container.
    fn teardown_port_forward(&self, teardown_pf: TeardownPortForward) -> NetavarkResult<()>;
}

/// Types of firewall backend
enum FirewallImpl {
    Fwnone,
}

/// What firewall implementations does this system support?
fn get_firewall_impl() -> NetavarkResult<FirewallImpl> {
    // Forcibly disable firewall features
    debug!("Firewall is forcibly disabled");
    return Ok(FirewallImpl::Fwnone);
}

/// Get the preferred firewall implementation for the current system
/// configuration.
pub fn get_supported_firewall_driver() -> NetavarkResult<Box<dyn FirewallDriver>> {
    match get_firewall_impl() {
        Ok(fw) => match fw {
            FirewallImpl::Fwnone => {
                info!("Not using firewall");
                fwnone::new()
            },
        },
        Err(e) => Err(e),
    }
}
