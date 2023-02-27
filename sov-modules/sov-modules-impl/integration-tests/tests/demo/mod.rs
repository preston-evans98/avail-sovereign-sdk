// TODO remove it after all the macors are implemented.
#![allow(dead_code)]

use example_election::Election;
use example_value_setter::ValueSetter;
use sov_modules_api::{
    mocks::{MockContext, MockPublicKey},
    CallResponse, Context, DispatchCall, DispatchQuery, Error, Genesis, Module,
};
use sov_modules_macros::{DispatchCall, DispatchQuery, Genesis};
use sov_state::{CacheLog, ValueReader};
use sovereign_db::state_db::StateDB;
use sovereign_sdk::serial::{Decode, Encode};
use std::{io::Cursor, marker::PhantomData};

/// dispatch_tx is a high level interface used by the sdk.
/// Transaction signature must be checked outside of this function.
fn dispatch_tx<C: Context, VR: ValueReader>(
    _tx_data: Vec<u8>,
    _context: C,
    _value_reader: VR,
) -> Result<(CallResponse, CacheLog), Error> {
    // 1. Create Storage (with fresh Cache)
    // 2. Deserialize tx
    // 3. deserialized_tx.dispatch(...)
    todo!()
}

/// Runtime defines modules registered in the rollup.
// #[derive(Genesis, DispatchCall, DispatchQuery, Client)]
#[derive(Genesis, DispatchCall, DispatchQuery)]
struct Runtime<C: Context> {
    election: Election<C>,
    value_adder: ValueSetter<C>,
    //..
}

fn decode_dispatchable<C: Context>(
    data: Vec<u8>,
) -> Result<impl DispatchCall<Context = C>, anyhow::Error> {
    let mut data = Cursor::new(data);
    Ok(RuntimeCall::<C>::decode(&mut data)?)
}

// This code will be generated by a macro
// Possible clients:
// - test client
// - rest api client
// - wasm bindings
// - json abi
#[derive(Default)]
struct Client<C: Context> {
    _phantom: PhantomData<C>,
}

// This code will be generated by a macro
impl<C: Context> Client<C> {
    fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    fn send_election_message(&self, data: <Election<C> as Module>::CallMessage) -> Vec<u8> {
        let call = RuntimeCall::<C>::election(data);
        call.encode_to_vec()
    }

    fn send_value_adder_message(&self, data: <ValueSetter<C> as Module>::CallMessage) -> Vec<u8> {
        let call = RuntimeCall::<C>::value_adder(data);
        call.encode_to_vec()
    }

    fn query_election(&self, data: <Election<C> as Module>::QueryMessage) -> Vec<u8> {
        let query = RuntimeQuery::<C>::election(data);
        query.encode_to_vec()
    }

    fn query_value_adder(&self, data: <ValueSetter<C> as Module>::QueryMessage) -> Vec<u8> {
        let query = RuntimeQuery::<C>::value_adder(data);
        query.encode_to_vec()
    }
}

fn decode_queryable<C: Context>(
    data: Vec<u8>,
) -> Result<impl DispatchQuery<Context = C>, anyhow::Error> {
    let mut data = Cursor::new(data);
    Ok(RuntimeQuery::<C>::decode(&mut data)?)
}

#[test]
fn test_demo() {
    let client = Client::<C>::new();
    type C = MockContext;
    let sender = MockPublicKey::try_from("admin").unwrap();
    let context = MockContext::new(sender);
    let temp_db = StateDB::temporary();
    let storage = Runtime::<C>::genesis(temp_db).unwrap();

    // Call the election module.
    {
        let call_message = example_election::call::CallMessage::<C>::SetCandidates {
            names: vec!["candidate_1".to_owned()],
        };

        let serialized_message = client.send_election_message(call_message);
        let module = decode_dispatchable::<C>(serialized_message).unwrap();
        let result = module.dispatch_call(storage.clone(), &context);
        assert!(result.is_ok())
    }

    // Query the election module.
    {
        let query_message = example_election::query::QueryMessage::Result;

        let serialized_message = client.query_election(query_message);
        let module = decode_queryable::<C>(serialized_message).unwrap();

        let response = module.dispatch_query(storage);
        let _json_response = std::str::from_utf8(&response.response).unwrap();
    }
}