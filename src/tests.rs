use crate::{
    cache::{consts::ROLE_PREFIX, players::PlayerSplitsData, split::Split},
    utils::{
        extract_name_or_uuid_and_splits_from_line::extract_name_or_uuid_and_splits_from_line,
        extract_split_from_pb_role_name::extract_split_from_pb_role_name,
        extract_split_from_role_name::extract_split_from_role_name,
    },
};

#[test]
pub fn test_extract_split_from_role_name() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        extract_split_from_role_name(format!("{}SS9:4", ROLE_PREFIX).as_str())?,
        (Split::SecondStructure, 9, 40)
    );
    assert_eq!(
        extract_split_from_role_name(format!("{}SS10:4", ROLE_PREFIX).as_str())?,
        (Split::SecondStructure, 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name(format!("{}E10:4", ROLE_PREFIX).as_str())?,
        (Split::EyeSpy, 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name(format!("{}EE10:4", ROLE_PREFIX).as_str())?,
        (Split::EndEnter, 10, 40)
    );
    Ok(())
}

#[test]
pub fn test_extract_split_from_pb_role_name() {
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}FSPB", ROLE_PREFIX).as_str()),
        Some(Split::FirstStructure)
    );
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}SSPB", ROLE_PREFIX).as_str()),
        Some(Split::SecondStructure)
    );
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}BPB", ROLE_PREFIX).as_str()),
        Some(Split::Blind)
    );
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}EPB", ROLE_PREFIX).as_str()),
        Some(Split::EyeSpy)
    );
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}EEPB", ROLE_PREFIX).as_str()),
        Some(Split::EndEnter)
    );
}

#[test]
pub fn test_extract_name_and_splits_from_line() -> Result<(), Box<dyn std::error::Error>> {
    let mut split_data = PlayerSplitsData {
        first_structure: 10,
        second_structure: 20,
        blind: 30,
        eye_spy: 40,
        end_enter: 50,
        finish: None,
    };
    assert_eq!(
        extract_name_or_uuid_and_splits_from_line("SathyaPramodh: 10/20/30/40/50")?,
        ("SathyaPramodh".to_string(), split_data)
    );
    assert_eq!(
        extract_name_or_uuid_and_splits_from_line("name_name_: 10/20/30/40/50")?,
        ("name_name_".to_string(), split_data)
    );

    split_data.finish = Some(60);
    assert_eq!(
        extract_name_or_uuid_and_splits_from_line("SathyaPramodh: 10/20/30/40/50/60")?,
        ("SathyaPramodh".to_string(), split_data)
    );
    assert_eq!(
        extract_name_or_uuid_and_splits_from_line("name_name_: 10/20/30/40/50/60")?,
        ("name_name_".to_string(), split_data)
    );
    Ok(())
}
