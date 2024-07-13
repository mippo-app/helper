use std::collections::HashMap;
use uuid::Uuid;

pub mod fs;

/*
pub mod basic {
  tonic::include_proto!("basic");
}
*/

pub trait PbPrimitive {
    type MyType;
    type PbType;
    // fn from_pb(s: Option<Self::PbType>) -> Option<Self::MyType>;
    // fn to_pb(&self) -> Self::PbType;
}

pub fn conv_pb_multi_to_multi<T, V>(pbs: Vec<T>) -> Vec<V>
where
    T: From<V>,
    V: From<T>,
{
    let mut r = Vec::with_capacity(pbs.len());

    for a in pbs.into_iter() {
        let abc = V::from(a);

        r.push(abc);
    }

    return r;
}
pub fn conv_multi_to_multi_pb<T, V>(pbs: Vec<T>) -> Vec<V>
where
    T: From<V>,
    V: From<T>,
{
    let mut r = Vec::with_capacity(pbs.len());

    for a in pbs.into_iter() {
        let abc = V::from(a);
        r.push(abc);
    }

    return r;
}

/*pub fn conv_pb_multi_to_multi<T>(pbs: Vec<T::PbType>) -> Vec<T::MyType>
where
  T: PbPrimitive,
{
  let mut r = vec![];

  for a in pbs.into_iter() {
    let abc = T::from_pb(Some(a));

    if let Some(d) = abc {
      r.push(d);
    }
  }

  return r;
}*/

pub fn slice_to_uuid(uuid: &Vec<u8>) -> uuid::Uuid {
    return uuid::Uuid::from_slice(&uuid).unwrap();
}

pub fn uuid_to_slice(u: uuid::Uuid) -> Vec<u8> {
    return u.as_bytes().to_vec();
}

pub fn conv_pb_uuid_to_uuid_one(m: Vec<u8>) -> Uuid {
    return slice_to_uuid(&m);
}

pub fn conv_uuid_one_to_uuid_pb(m: Uuid) -> Vec<u8> {
    return uuid_to_slice(m);
}

pub fn conv_uuid_pb_to_uuid_multi(m: Vec<Vec<u8>>) -> Vec<Uuid> {
    let mut r = Vec::with_capacity(m.len());
    for a in m.into_iter() {
        r.push(conv_pb_uuid_to_uuid_one(a));
    }

    return r;
}

pub fn conv_uuid_multi_to_uuid_pb(m: Vec<Uuid>) -> Vec<Vec<u8>> {
    let mut r = Vec::with_capacity(m.len());
    for a in m.into_iter() {
        r.push(conv_uuid_one_to_uuid_pb(a));
    }

    return r;
}

pub fn conv_pb_map_to_map_one<T, V>(m: HashMap<String, T>) -> HashMap<String, V>
where
    T: From<V>,
    V: From<T>,
{
    let mut r = HashMap::new();

    for (k, v) in m {
        let a = V::from(v);

        r.insert(k.clone(), a);
    }

    return r;
}

pub fn conv_map_one_to_map_pb<T, V>(m: HashMap<String, T>) -> HashMap<String, V>
where
    T: From<V>,
    V: From<T>,
{
    let mut r = HashMap::new();

    for (k, v) in m {
        let a = V::from(v);

        r.insert(k.clone(), a);
    }

    return r;
}

pub fn conv_option_one_to_option_pb<T, V>(one: Option<T>) -> Option<V>
where
    T: From<V>,
    V: From<T>,
{
    if let Some(a) = one {
        return Some(V::from(a));
    }

    return None;
}

pub fn conv_option_pb_to_option_one<T, V>(one: Option<T>) -> Option<V>
where
    T: From<V>,
    V: From<T>,
{
    if let Some(one) = one {
        return Some(V::from(one));
    } else {
        return None;
    }
}

/*
pub fn conv_pb_map_to_map_one<T>(m: HashMap<String, T::PbType>) -> HashMap<String, T::MyType>
where
  T: PbPrimitive,
{
  let mut r = HashMap::new();

  for (k, v) in m {
    let a = T::from_pb(Some(v));

    if let Some(b) = a {
      r.insert(k.clone(), b);
    }
  }

  return r;
}*/

/*
pub fn conv_map_one_to_map_pb<T>(m: HashMap<String, T>) -> HashMap<String, T::PbType>
where
  T: PbPrimitive,
{
  let mut r = HashMap::new();

  for (k, v) in m {
    let a = v.to_pb();

    r.insert(k.clone(), a);
  }

  return r;
}

pub fn conv_multi_to_multi_pb<T>(pbs: Vec<T>) -> Vec<T::PbType>
where
  T: PbPrimitive,
{
  let mut r = vec![];

  for a in pbs.into_iter() {
    r.push(a.to_pb());
  }

  return r;
}

pub fn conv_option_one_to_option_pb<T>(one: Option<T>) -> Option<T::PbType>
where
  T: PbPrimitive,
{
  if let Some(a) = one {
    return Some(a.to_pb());
  }

  return None;
}

pub fn conv_option_pb_to_option_one<T>(one: Option<T::PbType>) -> Option<T::MyType>
where
  T: PbPrimitive,
{
  T::from_pb(one)
}
*/
