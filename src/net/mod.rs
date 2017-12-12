// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

pub use self::listener::Listener;
pub use self::peer::{Acceptor, BootstrapAcceptError, BootstrapAcceptor, BootstrapError,
                     ConnectError, ConnectHandshakeError, ExternalReachability, P2pConnectionInfo,
                     Peer, PeerError, PrivConnectionInfo, PubConnectionInfo,
                     RendezvousConnectError, SingleConnectionError, Uid, bootstrap};
pub use self::protocol_agnostic::{PaAddr, PaIncoming, PaListener, PaRendezvousConnectError,
                                  PaStream, UtpRendezvousConnectError};
pub use self::service_discovery::ServiceDiscovery;
pub use self::socket::{MAX_PAYLOAD_SIZE, Priority, Socket, SocketError};

mod listener;
pub mod peer;
mod socket;
mod protocol_agnostic;
pub mod service_discovery;
