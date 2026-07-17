use super::span::SourceSpan;
use xray_error::XRayResult;

/// One comma-separated condition-list branch.
#[derive(Clone, Debug, PartialEq)]
pub struct CondlistBranch {
  pub conditions: Vec<CondlistCondition>,
  pub effects: Vec<CondlistCondition>,
  pub result: Option<String>,
  pub span: SourceSpan,
}

/// A condition or effect in a condition-list branch.
#[derive(Clone, Debug, PartialEq)]
pub enum CondlistCondition {
  InfoPortion {
    name: String,
    required: bool,
    span: SourceSpan,
  },
  Probability {
    value: f64,
    span: SourceSpan,
  },
  Function {
    expected: bool,
    name: String,
    parameters: Option<Vec<String>>,
    span: SourceSpan,
  },
}

impl CondlistBranch {
  pub fn parse(branch: &str, branch_offset: usize) -> XRayResult<CondlistBranch> {
    let mut cursor: usize = 0;
    let mut conditions: Vec<CondlistCondition> = Vec::new();

    if Self::byte_at(branch, cursor) == Some(b'{') {
      let conditions_start: usize = cursor + 1;
      let conditions_end: usize =
        Self::find_delimiter(branch, conditions_start, b'}').ok_or_else(|| {
          SourceSpan::parsing_error(
            branch_offset + cursor,
            branch_offset + cursor + 1,
            "Expected closing '}' for the condition list",
          )
        })?;

      conditions = Self::parse_conditions(
        &branch[conditions_start..conditions_end],
        branch_offset + conditions_start,
      )?;
      cursor = conditions_end + 1;
    }

    Self::skip_whitespace(branch, &mut cursor);

    let result_start: usize = cursor;

    while let Some(byte) = Self::byte_at(branch, cursor) {
      if matches!(byte, b'{' | b'}') {
        return Err(SourceSpan::parsing_error(
          branch_offset + cursor,
          branch_offset + cursor + 1,
          "Unexpected brace in condlist result",
        ));
      }

      if byte == b'%' {
        break;
      }

      cursor += 1;
    }

    let result: Option<String> = (!branch[result_start..cursor].trim().is_empty())
      .then(|| branch[result_start..cursor].trim().to_owned());

    if Self::byte_at(branch, cursor) != Some(b'%') {
      if result.is_some() {
        return Ok(CondlistBranch {
          conditions,
          effects: Vec::new(),
          result,
          span: SourceSpan::new(branch_offset, branch_offset + branch.len()),
        });
      }

      return Err(SourceSpan::parsing_error(
        branch_offset + result_start,
        branch_offset + result_start,
        "Expected a result or effect list after the condition list",
      ));
    }

    let effects_start: usize = cursor + 1;
    let effects_end: usize =
      Self::find_delimiter(branch, effects_start, b'%').ok_or_else(|| {
        SourceSpan::parsing_error(
          branch_offset + cursor,
          branch_offset + cursor + 1,
          "Expected closing '%' for the effect list",
        )
      })?;
    let effects: Vec<CondlistCondition> = Self::parse_conditions(
      &branch[effects_start..effects_end],
      branch_offset + effects_start,
    )?;

    if !branch[effects_end + 1..].trim().is_empty() {
      return Err(SourceSpan::parsing_error(
        branch_offset + effects_end + 1,
        branch_offset + branch.len(),
        "Unexpected data after the effect list",
      ));
    }

    Ok(CondlistBranch {
      conditions,
      effects,
      result,
      span: SourceSpan::new(branch_offset, branch_offset + branch.len()),
    })
  }

  fn parse_conditions(value: &str, value_offset: usize) -> XRayResult<Vec<CondlistCondition>> {
    let mut conditions: Vec<CondlistCondition> = Vec::new();
    let mut cursor: usize = 0;

    while cursor < value.len() {
      Self::skip_whitespace(value, &mut cursor);

      if cursor == value.len() {
        break;
      }

      conditions.push(Self::parse_condition(value, value_offset, &mut cursor)?);
    }

    Ok(conditions)
  }

  fn parse_condition(
    value: &str,
    value_offset: usize,
    cursor: &mut usize,
  ) -> XRayResult<CondlistCondition> {
    let token_start: usize = *cursor;
    let sign: u8 = Self::byte_at(value, *cursor).expect("Cursor should point to a condition token");

    if !Self::is_condition_sign(sign) {
      return Err(SourceSpan::parsing_error(
        value_offset + *cursor,
        value_offset + *cursor + 1,
        "Expected a condition or effect prefix ('+', '-', '~', '=', or '!')",
      ));
    }

    *cursor += 1;
    let name_start: usize = *cursor;
    let mut has_function_call: bool = false;
    let mut parameters: Option<Vec<String>> = None;

    while let Some(byte) = Self::byte_at(value, *cursor) {
      if byte.is_ascii_whitespace() || Self::is_condition_sign(byte) {
        break;
      }

      if matches!(byte, b',' | b'{' | b'}' | b'%') {
        return Err(SourceSpan::parsing_error(
          value_offset + *cursor,
          value_offset + *cursor + 1,
          "Unexpected delimiter in a condition or effect",
        ));
      }

      if byte == b')' {
        return Err(SourceSpan::parsing_error(
          value_offset + *cursor,
          value_offset + *cursor + 1,
          "Unexpected ')' in a condition or effect",
        ));
      }

      if byte == b'(' {
        if *cursor == name_start || has_function_call {
          return Err(SourceSpan::parsing_error(
            value_offset + *cursor,
            value_offset + *cursor + 1,
            "Expected one function call after a condition or effect name",
          ));
        }

        if !matches!(sign, b'=' | b'!') {
          return Err(SourceSpan::parsing_error(
            value_offset + token_start,
            value_offset + *cursor + 1,
            "Only '=' and '!' tokens can call functions",
          ));
        }

        has_function_call = true;
        let (next_cursor, parsed_parameters): (usize, Vec<String>) =
          Self::parse_function_call(value, *cursor, value_offset)?;
        *cursor = next_cursor;
        parameters = Some(parsed_parameters);

        if let Some(next) = Self::byte_at(value, *cursor)
          && !next.is_ascii_whitespace()
          && !Self::is_condition_sign(next)
        {
          return Err(SourceSpan::parsing_error(
            value_offset + *cursor,
            value_offset + *cursor + 1,
            "Unexpected data after function call",
          ));
        }

        continue;
      }

      *cursor += 1;
    }

    if name_start == *cursor {
      return Err(SourceSpan::parsing_error(
        value_offset + token_start,
        value_offset + token_start + 1,
        "Expected a name after condition or effect prefix",
      ));
    }

    let name: String = value[name_start..if has_function_call {
      Self::find_function_start(value, name_start)
    } else {
      *cursor
    }]
      .to_owned();
    let span: SourceSpan = SourceSpan::new(value_offset + token_start, value_offset + *cursor);

    match sign {
      b'+' => Ok(CondlistCondition::InfoPortion {
        name,
        required: true,
        span,
      }),
      b'-' => Ok(CondlistCondition::InfoPortion {
        name,
        required: false,
        span,
      }),
      b'~' => match name.parse::<f64>() {
        Ok(value) => Ok(CondlistCondition::Probability { value, span }),
        Err(_) => Err(SourceSpan::parsing_error(
          value_offset + name_start,
          value_offset + *cursor,
          "Expected a numeric probability after '~'",
        )),
      },
      b'=' => Ok(CondlistCondition::Function {
        expected: true,
        name,
        parameters,
        span,
      }),
      b'!' => Ok(CondlistCondition::Function {
        expected: false,
        name,
        parameters,
        span,
      }),
      _ => unreachable!("Condition signs are checked above"),
    }
  }

  fn find_function_start(value: &str, name_start: usize) -> usize {
    value[name_start..]
      .find('(')
      .map(|index| name_start + index)
      .expect("Function calls are checked while parsing")
  }

  fn parse_function_call(
    value: &str,
    open_parenthesis: usize,
    value_offset: usize,
  ) -> XRayResult<(usize, Vec<String>)> {
    let mut cursor: usize = open_parenthesis + 1;

    while let Some(byte) = Self::byte_at(value, cursor) {
      if byte == b')' {
        let parameters: Vec<String> = value[open_parenthesis + 1..cursor]
          .split(':')
          .filter(|parameter| !parameter.is_empty())
          .map(str::to_owned)
          .collect();

        return Ok((cursor + 1, parameters));
      }

      if byte.is_ascii_whitespace() || matches!(byte, b'(' | b',' | b'{' | b'}' | b'%') {
        return Err(SourceSpan::parsing_error(
          value_offset + cursor,
          value_offset + cursor + 1,
          "Unexpected character in function parameters",
        ));
      }

      cursor += 1;
    }

    Err(SourceSpan::parsing_error(
      value_offset + open_parenthesis,
      value_offset + open_parenthesis + 1,
      "Expected closing ')' for function call",
    ))
  }

  fn find_delimiter(value: &str, start: usize, delimiter: u8) -> Option<usize> {
    value.as_bytes()[start..]
      .iter()
      .position(|byte| *byte == delimiter)
      .map(|position| start + position)
  }

  fn skip_whitespace(value: &str, cursor: &mut usize) {
    while Self::byte_at(value, *cursor).is_some_and(|byte| byte.is_ascii_whitespace()) {
      *cursor += 1;
    }
  }

  fn byte_at(value: &str, index: usize) -> Option<u8> {
    value.as_bytes().get(index).copied()
  }

  fn is_condition_sign(value: u8) -> bool {
    matches!(value, b'+' | b'-' | b'~' | b'=' | b'!')
  }
}
