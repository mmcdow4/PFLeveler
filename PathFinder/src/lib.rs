pub mod ability_scores;
pub mod pf_character;
pub mod character_class;
pub mod equipment;
pub mod feat;
pub mod xlsx_formats;
pub mod utilities;
pub mod pf_table;
pub mod race;
pub mod skill;
pub mod spell;
pub mod error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
