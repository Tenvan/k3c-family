//! Argumente eines Aufrufs (JSON-Array) als Tupel, mit den Fehlertexten von Go `decodeArgs`.

use std::cell::Cell;
use std::future::Future;

use serde::de::value::Error;
use serde::de::{DeserializeOwned, DeserializeSeed, Deserializer, Error as _, SeqAccess, Visitor};
use serde::forward_to_deserialize_any;
use serde_json::Value;

tokio::task_local! {
    /// Ob [`args`] im laufenden Aufruf gescheitert ist: die HTTP-API antwortet dann 400 statt 422.
    static BAD_ARGS: Cell<bool>;
}

/// Führt einen Aufruf aus und meldet, ob seine Argumente abgelehnt wurden.
pub(super) async fn watch<F: Future>(call: F) -> (F::Output, bool) {
    BAD_ARGS
        .scope(Cell::new(false), async {
            let out = call.await;
            (out, BAD_ARGS.with(Cell::get))
        })
        .await
}

/// Liest die Argumente als `T`: `()` ohne Argument, `(A,)` mit einem, `(A, B)` mit zweien. Anzahl und Typ müssen passen.
pub fn args<T: DeserializeOwned>(raw: Vec<Value>) -> Result<T, String> {
    T::deserialize(Args(raw)).map_err(|e| {
        // Außerhalb eines HTTP-Aufrufs (Tauri-Command) gibt es nichts zu merken.
        let _ = BAD_ARGS.try_with(|bad| bad.set(true));
        e.to_string()
    })
}

struct Args(Vec<Value>);

impl Args {
    fn expect(&self, len: usize) -> Result<(), Error> {
        if self.0.len() == len {
            return Ok(());
        }
        let got = self.0.len();
        Err(Error::custom(format!("erwartet {len} Argumente, bekommen {got}")))
    }
}

impl<'de> Deserializer<'de> for Args {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_seq(Elements {
            rest: self.0.into_iter(),
            number: 0,
        })
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.expect(0)?;
        visitor.visit_unit()
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Error> {
        self.expect(len)?;
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.deserialize_tuple(len, visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit_struct
        newtype_struct seq map struct enum identifier ignored_any
    }
}

struct Elements {
    rest: std::vec::IntoIter<Value>,
    number: usize,
}

impl<'de> SeqAccess<'de> for Elements {
    type Error = Error;

    fn next_element_seed<S: DeserializeSeed<'de>>(&mut self, seed: S) -> Result<Option<S::Value>, Error> {
        let Some(value) = self.rest.next() else {
            return Ok(None);
        };
        self.number += 1;
        let number = self.number;
        seed.deserialize(value)
            .map(Some)
            .map_err(|e| Error::custom(format!("ungültiges Argument {number}: {e}")))
    }
}
