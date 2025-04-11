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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_pf_table() {
        let original_filename = String::from("E:\\dev\\PFLeveler\\cfg\\PathFinder.db");
        let copy_filename = String::from("E:\\dev\\PFLeveler\\cfg\\PathFinder_copy.db");
        
        // Delete the test copy if it already exists
        if fs::exists(&copy_filename).unwrap() {
            fs::remove_file(&copy_filename).unwrap();
        }

        // Load the database
        let mut table = pf_table::PfTable::new();
        table.read_file(&original_filename);
        // let table = pf_table::init_pf_table(&original_filename);

        // Write the copy
        table.write_file(&copy_filename);

        // Read database binaries and verify they are identical
        let mut copy_table = pf_table::PfTable::new();
        copy_table.read_file(&copy_filename);

        // assert_eq!(table, copy_table, "Database Files Differ!");
        
        assert_eq!(table.get_spells(), copy_table.get_spells(), "Spell Tables Differ!");
        
        assert_eq!(table.get_feats(), copy_table.get_feats(), "Feat Tables Differ!");
        
        assert_eq!(table.get_races(), copy_table.get_races(), "Race Tables Differ!");
        
        assert_eq!(table.get_class_features(), copy_table.get_class_features(), "Class Features Tables Differ!");
        
        assert_eq!(table.get_class_abilities(), copy_table.get_class_abilities(), "Class Abilities Tables Differ!");
        
        assert_eq!(table.get_class_choices(), copy_table.get_class_choices(), "Class Choices Tables Differ!");
        
        assert_eq!(table.get_classes(), copy_table.get_classes(), "Class Tables Differ!");
        
        assert_eq!(table.get_general_items(), copy_table.get_general_items(), "General Item Tables Differ!");
        
        assert_eq!(table.get_armor(), copy_table.get_armor(), "Armor Tables Differ!");
        
        assert_eq!(table.get_weapons(), copy_table.get_weapons(), "Weapon Tables Differ!");
        
        assert_eq!(table.get_skills(), copy_table.get_skills(), "Skill Tables Differ!");
        if fs::exists(&copy_filename).unwrap() {
            fs::remove_file(&copy_filename).unwrap();
        }
    }
}