use ff::PrimeField;
use group::GroupEncoding;
use serde::{de::Error as DeserializeError, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    curves::{EpAffine, EqAffine},
    fields::{Fp, Fq},
};

const ERR_CODE: &str = "deserialized bytes don't encode a field element";

impl Serialize for Fp {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_repr().serialize(s)
    }
}

impl<'de> Deserialize<'de> for Fp {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(d)?;
        match Fp::from_repr(bytes).into() {
            Some(fp) => Ok(fp),
            None => Err(D::Error::custom(ERR_CODE)),
        }
    }
}

impl Serialize for Fq {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_repr().serialize(s)
    }
}

impl<'de> Deserialize<'de> for Fq {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(d)?;
        match Fq::from_repr(bytes).into() {
            Some(fq) => Ok(fq),
            None => Err(D::Error::custom(ERR_CODE)),
        }
    }
}

impl Serialize for EpAffine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_bytes().serialize(s)
    }
}

impl<'de> Deserialize<'de> for EpAffine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(d)?;
        match EpAffine::from_bytes_unchecked(&bytes).into() {
            Some(ep_affine) => Ok(ep_affine),
            None => Err(D::Error::custom(ERR_CODE)),
        }
    }
}

impl Serialize for EqAffine {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.to_bytes().serialize(s)
    }
}

impl<'de> Deserialize<'de> for EqAffine {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(d)?;
        match EqAffine::from_bytes_unchecked(&bytes).into() {
            Some(eq_affine) => Ok(eq_affine),
            None => Err(D::Error::custom(ERR_CODE)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::fmt::Debug;

    use ff::Field;
    use group::{prime::PrimeCurveAffine, Curve, Group};
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    use crate::curves::{Ep, Eq};

    fn test_roundtrip<T: Serialize + for<'a> Deserialize<'a> + Debug + PartialEq>(t: &T) {
        //dbg!(t);
        let ser = serde_json::to_vec(t).unwrap();
        //dbg!(std::str::from_utf8(&ser));
        assert_eq!(*t, serde_json::from_slice(&ser).unwrap());
    }

    #[test]
    fn serde_fp() {
        let mut rng = XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        for _ in 0..100 {
            let f = Fp::random(&mut rng);
            test_roundtrip(&f);
        }

        let f = Fp::zero();
        test_roundtrip(&f);
        assert_eq!(
            serde_json::from_slice::<Fp>(
                b"[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
            )
            .unwrap(),
            f
        );

        let f = Fp::one();
        test_roundtrip(&f);
        assert_eq!(
            serde_json::from_slice::<Fp>(
                b"[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
            )
            .unwrap(),
            f
        );
    }

    #[test]
    fn serde_fq() {
        let mut rng = XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        for _ in 0..100 {
            let f = Fq::random(&mut rng);
            test_roundtrip(&f);
        }

        let f = Fq::zero();
        test_roundtrip(&f);
        assert_eq!(
            serde_json::from_slice::<Fq>(
                b"[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
            )
            .unwrap(),
            f
        );

        let f = Fq::one();
        test_roundtrip(&f);
        assert_eq!(
            serde_json::from_slice::<Fq>(
                b"[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
            )
            .unwrap(),
            f
        );
    }

    #[test]
    fn serde_ep_affine() {
        let mut rng = XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        for _ in 0..100 {
            let f = Ep::random(&mut rng);
            test_roundtrip(&f.to_affine());
        }

        let f = EpAffine::identity();
        test_roundtrip(&f);
        assert_eq!(
            serde_json::from_slice::<EpAffine>(
                b"[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
            )
            .unwrap(),
            f
        );

        let f = EpAffine::generator();
        test_roundtrip(&f);
        assert_eq!(
           serde_json::from_slice::<EpAffine>(
               b"[0,0,0,0,237,48,45,153,27,249,76,9,252,152,70,34,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64]"
           )
           .unwrap(),
           f
        );
    }

    #[test]
    fn serde_eq_affine() {
        let mut rng = XorShiftRng::from_seed([
            0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06,
            0xbc, 0xe5,
        ]);

        for _ in 0..100 {
            let f = Eq::random(&mut rng);
            test_roundtrip(&f.to_affine());
        }

        let f = EqAffine::identity();
        test_roundtrip(&f);
        assert_eq!(
            serde_json::from_slice::<EqAffine>(
                b"[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"
            )
            .unwrap(),
            f
        );

        let f = EqAffine::generator();
        test_roundtrip(&f);
        assert_eq!(
           serde_json::from_slice::<EqAffine>(
               b"[0,0,0,0,33,235,70,140,221,168,148,9,252,152,70,34,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,64]"
           )
           .unwrap(),
           f
       );
    }
}
