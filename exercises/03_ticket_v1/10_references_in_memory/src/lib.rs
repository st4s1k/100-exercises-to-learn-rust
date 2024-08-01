pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn u16_ref_size() {
        let architecture = 64 /* bit */;
        let reference_pointer_bit_size = 1 * architecture;
        let total_byte_size = reference_pointer_bit_size / 8;
        assert_eq!(size_of::<&u16>(), total_byte_size /* 64 bits or 8 bytes */);
    }

    #[test]
    fn u64_mut_ref_size() {
        let architecture = 64 /* bit */;
        let reference_pointer_bit_size = 1 * architecture;
        let total_byte_size = reference_pointer_bit_size / 8;
        assert_eq!(size_of::<&mut u64>(), total_byte_size /* 64 bits or 8 bytes */);
    }

    #[test]
    fn ticket_ref_size() {
        let architecture = 64 /* bit */;
        let reference_pointer_bit_size = 1 * architecture;
        let total_byte_size = reference_pointer_bit_size / 8;
        assert_eq!(size_of::<&Ticket>(), total_byte_size /* 64 bits or 8 bytes */);
    }
}
