mod dns_proxy {
    dharitri_sc::imports!();

    #[dharitri_sc::proxy]
    pub trait Dns {
        #[payable("MOA")]
        #[endpoint]
        fn register(&self, name: &ManagedBuffer);
    }
}

dharitri_sc::imports!();

/// Standard smart contract module that deals with registering usernames in a DNS contract.
///
/// Dharitri usernames/herotags need to be requested by the beneficiary.
/// For a contract, this means that they need an endpoint via which to request a username from the DNS.
///
#[dharitri_sc::module]
pub trait DnsModule {
    #[proxy]
    fn dns_proxy(&self, to: ManagedAddress) -> dns_proxy::Proxy<Self::Api>;

    #[payable("MOA")]
    #[only_owner]
    #[endpoint(dnsRegister)]
    fn dns_register(&self, dns_address: ManagedAddress, name: ManagedBuffer) {
        let payment = self.call_value().moa_value().clone_value();
        self.dns_proxy(dns_address)
            .register(&name)
            .with_moa_transfer(payment)
            .async_call()
            .call_and_exit()
    }
}
