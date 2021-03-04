pub mod api;

use std::{cell::RefCell, convert::TryInto, io, rc::Rc, vec::IntoIter};

use uptown_funk::{Executor, FromWasm, ToWasm};

#[derive(Clone)]
pub struct Resolver {
    iter: Rc<RefCell<IntoIter<smol::net::SocketAddr>>>,
}

impl Resolver {
    pub async fn resolve(name: &str) -> Result<Self, io::Error> {
        let resolved = smol::net::resolve(name).await?;
        Ok(Resolver {
            iter: Rc::new(RefCell::new(resolved.into_iter())),
        })
    }

    pub fn next(&self) -> Option<smol::net::SocketAddr> {
        self.iter.as_ref().borrow_mut().next()
    }
}

impl FromWasm for Resolver {
    type From = u32;
    type State = api::TcpState;

    fn from(
        state: &mut Self::State,
        _: &impl Executor,
        resolver_id: u32,
    ) -> Result<Self, uptown_funk::Trap>
    where
        Self: Sized,
    {
        match state.resolvers.get(resolver_id) {
            Some(resolver) => Ok(resolver.clone()),
            None => Err(uptown_funk::Trap::new("TcpListener not found")),
        }
    }
}

enum ResolverResult {
    Ok(Resolver),
    Err(String),
}

impl ToWasm for ResolverResult {
    type To = u32;
    type State = api::TcpState;

    fn to(
        state: &mut Self::State,
        _: &impl Executor,
        result: Self,
    ) -> Result<u32, uptown_funk::Trap> {
        match result {
            ResolverResult::Ok(resolver) => Ok(state.resolvers.add(resolver)),
            ResolverResult::Err(_err) => Ok(0),
        }
    }
}

#[derive(Clone)]
pub struct TcpListener(smol::net::TcpListener);

impl TcpListener {
    pub async fn bind(addr: &[u8], port: u16) -> Result<Self, io::Error> {
        match addr.len() {
            4 => {
                let addr: [u8; 4] = addr.try_into().unwrap();
                let addr = smol::net::Ipv4Addr::from(addr);
                match smol::net::TcpListener::bind((addr, port)).await {
                    Ok(tcp_listener) => Ok(Self(tcp_listener)),
                    Err(err) => Err(err),
                }
            }
            16 => {
                let addr: [u8; 16] = addr.try_into().unwrap();
                let addr = smol::net::Ipv6Addr::from(addr);
                match smol::net::TcpListener::bind((addr, port)).await {
                    Ok(tcp_listener) => Ok(Self(tcp_listener)),
                    Err(err) => Err(err),
                }
            }
            _ => Err(io::Error::from_raw_os_error(22)), // Wrong argument error code.
        }
    }

    pub async fn accept(&self) -> Result<TcpStream, io::Error> {
        let (stream, _address) = self.0.accept().await?;
        Ok(TcpStream(stream))
    }
}

impl FromWasm for TcpListener {
    type From = u32;
    type State = api::TcpState;

    fn from(
        state: &mut Self::State,
        _: &impl Executor,
        tcp_listener_id: u32,
    ) -> Result<Self, uptown_funk::Trap>
    where
        Self: Sized,
    {
        match state.listeners.get(tcp_listener_id) {
            Some(tcp_listener) => Ok(tcp_listener.clone()),
            None => Err(uptown_funk::Trap::new("TcpListener not found")),
        }
    }
}

enum TcpListenerResult {
    Ok(TcpListener),
    Err(String),
}

impl ToWasm for TcpListenerResult {
    type To = u32;
    type State = api::TcpState;

    fn to(
        state: &mut Self::State,
        _: &impl Executor,
        result: Self,
    ) -> Result<u32, uptown_funk::Trap> {
        match result {
            TcpListenerResult::Ok(listener) => Ok(state.listeners.add(listener)),
            TcpListenerResult::Err(_err) => Ok(0),
        }
    }
}

#[derive(Clone)]
pub struct TcpStream(smol::net::TcpStream);

impl TcpStream {
    pub async fn connect(addr: &[u8], port: u16) -> Result<Self, io::Error> {
        match addr.len() {
            4 => {
                let addr: [u8; 4] = addr.try_into().unwrap();
                let addr = smol::net::Ipv4Addr::from(addr);
                match smol::net::TcpStream::connect((addr, port)).await {
                    Ok(tcp_stream) => Ok(Self(tcp_stream)),
                    Err(err) => Err(err),
                }
            }
            16 => {
                let addr: [u8; 16] = addr.try_into().unwrap();
                let addr = smol::net::Ipv6Addr::from(addr);
                match smol::net::TcpStream::connect((addr, port)).await {
                    Ok(tcp_stream) => Ok(Self(tcp_stream)),
                    Err(err) => Err(err),
                }
            }
            _ => Err(io::Error::from_raw_os_error(22)), // Wrong argument error code.
        }
    }
}

impl FromWasm for TcpStream {
    type From = u32;
    type State = api::TcpState;

    fn from(
        state: &mut Self::State,
        _: &impl Executor,
        tcp_stream_id: u32,
    ) -> Result<Self, uptown_funk::Trap>
    where
        Self: Sized,
    {
        match state.streams.get(tcp_stream_id) {
            Some(tcp_stream) => Ok(tcp_stream.clone()),
            None => Err(uptown_funk::Trap::new("TcpStream not found")),
        }
    }
}
enum TcpStreamResult {
    Ok(TcpStream),
    Err(String),
}

impl ToWasm for TcpStreamResult {
    type To = u32;
    type State = api::TcpState;

    fn to(
        state: &mut Self::State,
        _: &impl Executor,
        result: Self,
    ) -> Result<u32, uptown_funk::Trap> {
        match result {
            TcpStreamResult::Ok(stream) => Ok(state.streams.add(stream)),
            TcpStreamResult::Err(_err) => Ok(0),
        }
    }
}
