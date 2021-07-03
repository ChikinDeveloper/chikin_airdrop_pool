pub trait Packable {
    const PACKED_SIZE: usize;

    fn unpack(data: &[u8]) -> Result<Self, crate::error::AirdropPoolError> where Self: Sized;
    fn pack(&self) -> Vec<u8>;
    fn pack_into(&self, data: &mut [u8]) -> Result<(), crate::error::AirdropPoolError>;
}

#[macro_export]
macro_rules! implement_packable {
    ($for_type:ty, $packed_size:expr) => {
        impl Packable for $for_type {
            const PACKED_SIZE: usize = $packed_size;

            fn unpack(mut data: &[u8]) -> Result<Self, crate::error::AirdropPoolError> {
                if data.len() != Self::PACKED_SIZE {
                    // panic!("Failed to unpack type {}, len={}, expected={}", stringify!($for_type), data.len(), Self::PACKED_SIZE);
                    return Err(crate::error::AirdropPoolError::FailedToUnpackData);
                }
                assert_eq!(data.len(), Self::PACKED_SIZE);
                borsh::BorshDeserialize::deserialize(&mut data)
                    .map_err(|_| crate::error::AirdropPoolError::FailedToUnpackData)
            }

            fn pack(&self) -> Vec<u8> {
                let mut result = borsh::BorshSerialize::try_to_vec(self).unwrap();
                result.resize(Self::PACKED_SIZE, 0);
                result
            }

            fn pack_into(&self, data: &mut [u8]) -> Result<(), crate::error::AirdropPoolError> {
                if data.len() != Self::PACKED_SIZE {
                    // panic!("Failed to pack_into type {}, len={}, expected={}", stringify!($for_type), data.len(), Self::PACKED_SIZE);
                    return Err(crate::error::AirdropPoolError::FailedToPackData);
                }
                data.copy_from_slice(&self.pack());
                Ok(())
            }
        }
    };
}