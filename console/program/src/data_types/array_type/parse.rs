// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

// TODO (d0cd): Make this implementation iterative.
//  The use of recursion here introduces the possibility of a stack overflow.

impl<N: Network> Parser for ArrayType<N> {
    /// Parses a string into a literal type.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the opening brackets and following whitespaces.
        let (string, opening_brackets) = many1(pair(tag("["), Sanitizer::parse_whitespaces))(string)?;
        // Parse the element type.
        let (mut remaining_string, element_type) = ElementType::parse(string)?;
        // Count the number of opening brackets and parse the same number of dimensions.
        let mut dimensions = Vec::with_capacity(opening_brackets.len());
        for _ in 0..opening_brackets.len() {
            // Parse the whitespaces from the string.
            let (string, _) = Sanitizer::parse_whitespaces(remaining_string)?;
            // Parse the dimension from the string.
            let (string, dimension) =
                map_res(recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))), |digits: &str| {
                    digits.replace("_", "").parse::<u64>()
                })(string)?;
            dimensions.push(dimension);
            // Parse the semicolon.
            let (string, _) = tag(";")(string)?;
            // Parse the whitespaces from the string.
            let (string, _) = Sanitizer::parse_whitespaces(string)?;
            // Parse the closing bracket.
            let (string, _) = Sanitizer::parse_whitespaces(string)?;
            remaining_string = string;
        }
        // Return the array type.
        map_res(take(0usize), |_| ArrayType::new(element_type, dimensions))(string)
    }
}

impl<N: Network> FromStr for ArrayType<N> {
    type Err = Error;

    /// Returns an array type from a string literal.
    fn from_str(string: &str) -> Result<Self> {
        match Self::parse(string) {
            Ok((remainder, object)) => {
                // Ensure the remainder is empty.
                ensure!(remainder.is_empty(), "Failed to parse string. Found invalid character in: \"{remainder}\"");
                // Return the object.
                Ok(object)
            }
            Err(error) => bail!("Failed to parse string. {error}"),
        }
    }
}

impl<N: Network> Debug for ArrayType<N> {
    /// Prints the array type as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<N: Network> Display for ArrayType<N> {
    /// Prints the array type as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", self.element_type, self.length)
    }
}