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
    fn string_size() {
        assert_eq!(size_of::<String>(), string_byte_size());
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        let title_field_byte_size = string_byte_size();
        let description_field_byte_size = string_byte_size();
        let status_field_byte_size = string_byte_size();
        let ticket_byte_size = title_field_byte_size +
            description_field_byte_size +
            status_field_byte_size;
        assert_eq!(size_of::<Ticket>(), ticket_byte_size);
    }

    fn string_byte_size() -> usize {
        let architecture = 64;
        let pointer_bit_size = 1 * architecture;
        let length_bit_size = 1 * architecture;
        let capacity_bit_size = 1 * architecture;
        let total_bit_ize = pointer_bit_size + length_bit_size + capacity_bit_size;
        let total_byte_size = total_bit_ize / 8;
        total_byte_size
    }
}
