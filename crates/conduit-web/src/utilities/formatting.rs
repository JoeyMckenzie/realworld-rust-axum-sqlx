use super::errors::{ConduitWebError, ConduitWebResult};

pub fn convert_to_friend_date_string(utc_date: String) -> ConduitWebResult<String> {
    let as_datetime = chrono::DateTime::parse_from_rfc3339(&utc_date);

    if let Ok(parsed_datetime) = as_datetime {
        return Ok(parsed_datetime.format("%b %d").to_string());
    }

    Err(ConduitWebError::DateTimeInvalid)
}

#[cfg(test)]
mod convert_to_friend_date_string_should {
    use super::convert_to_friend_date_string;

    #[test]
    pub fn return_properly_formatted_date_when_input_is_valid() {
        // arrange
        let expected = "Jul 27".to_owned();
        let raw_date = "2022-07-27T05:17:02.122Z".to_owned();

        // act
        let result = convert_to_friend_date_string(raw_date);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    pub fn return_error_when_input_is_invalid_date() {
        // arrange
        let raw_date = "not a valid date".to_owned();

        // act
        let result = convert_to_friend_date_string(raw_date);

        // assert
        assert!(result.is_err());
    }
}
