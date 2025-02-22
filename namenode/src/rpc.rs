use std::sync::Arc;
use std::time::Duration;

// These modules/types should be defined elsewhere in your project.
use crate::v1;            // Assume this contains UMessage, UUri, UStatus, UPayloadFormat, etc.
use crate::datamodel;     // Assume datamodel::builder::Payload is defined here.
use crate::transport;     // Assume trait UTransport and type ListenHandle are defined here.

pub mod communication {
    use super::*;

    /// Callback function signature for implementing the RPC method.
    ///
    /// Callbacks can (optionally) return a Payload builder containing data to include
    /// in the response message. A response payload may be omitted only if no payload format
    /// was specified when creating the RpcServer.
    pub type RpcCallback = Box<dyn Fn(&v1::UMessage) -> Option<datamodel::builder::Payload> + Send + Sync>;

    /// Result type alias for creating an RpcServer.
    pub type ServerOrStatus = Result<RpcServer, v1::UStatus>;

    /// Interface for uEntities to receive and respond to RPC requests.
    ///
    /// Like all L2 client APIs, the RpcServer is a wrapper on top of the L1 UTransport API;
    /// in this instance, it is the request-handling half of the RPC model.
    pub struct RpcServer {
        /// Transport instance used for communication.
        transport: Arc<dyn transport::UTransport>,

        /// Optional TTL for responses.
        ttl: Option<Duration>,

        /// RPC callback method.
        callback: RpcCallback,

        /// Optional expected payload format.
        expected_payload_format: Option<v1::UPayloadFormat>,

        /// Handle to the connected callback for the RPC method wrapper.
        callback_handle: transport::ListenHandle,
    }

    impl RpcServer {
        /// Creates an RPC server.
        ///
        /// The callback will remain registered as long as the RpcServer is held.
        /// Dropping the RpcServer will automatically disconnect the callback.
        ///
        /// # Arguments
        ///
        /// * `transport` - Transport to offer the RPC method through.
        /// * `method_name` - URI representing the name clients will use to invoke the RPC method.
        /// * `callback` - Method to be called when requests are received.
        /// * `payload_format` - (Optional) Specifies the payload format expected for a non-empty response.
        /// * `ttl` - (Optional) Duration for which the response is valid from the moment `respond()` is called.
        ///
        /// # Returns
        ///
        /// * `Ok(RpcServer)` if the callback was connected successfully.
        /// * `Err(UStatus)` containing an error state otherwise.
        pub fn create(
            transport: Arc<dyn transport::UTransport>,
            method_name: &v1::UUri,
            callback: RpcCallback,
            payload_format: Option<v1::UPayloadFormat>,
            ttl: Option<Duration>,
        ) -> ServerOrStatus {
            let mut server = RpcServer::new(transport, payload_format, ttl, callback);
            let status = server.connect(method_name);
            if status.is_ok() {
                Ok(server)
            } else {
                Err(status)
            }
        }

        // Private constructor for RpcServer.
        fn new(
            transport: Arc<dyn transport::UTransport>,
            expected_payload_format: Option<v1::UPayloadFormat>,
            ttl: Option<Duration>,
            callback: RpcCallback,
        ) -> Self {
            // Initialize the callback handle. Here we assume ListenHandle implements Default.
            let callback_handle = transport::ListenHandle::default();
            RpcServer {
                transport,
                ttl,
                callback,
                expected_payload_format,
                callback_handle,
            }
        }

        /// Connects the RPC callback method and returns the status from the transport’s listener registration.
        ///
        /// # Arguments
        ///
        /// * `method` - URI representing the name clients will use to invoke the RPC method.
        ///
        /// # Returns
        ///
        /// * An "OK" UStatus if connected successfully; an error status otherwise.
        fn connect(&mut self, method: &v1::UUri) -> v1::UStatus {
            // Assume that the transport provides a method `register_listener` that takes
            // the method URI and a reference to the callback and returns a ListenHandle.
            self.callback_handle = self.transport.register_listener(method, &self.callback);
            // For demonstration, assume registration is always successful.
            v1::UStatus::ok()
        }
    }
}
