use simfony::witness::WitnessValues;
use simfony::{elements, CompiledProgram};

#[derive(Clone, Copy, Debug)]
pub struct Example {
    description: &'static str,
    program: &'static str,
    witness: &'static str,
    lock_time: u32,
    sequence: u32,
}

impl Example {
    pub fn description(self) -> &'static str {
        self.description
    }

    pub fn compiled(self) -> CompiledProgram {
        CompiledProgram::new(self.program).expect("example program should compile")
    }

    pub fn program_text(self) -> &'static str {
        self.program
    }

    pub fn witness_values(self) -> WitnessValues {
        serde_json::from_str(self.witness).expect("example witness should parse")
    }

    #[cfg(test)]
    pub fn satisfied(self) -> simfony::SatisfiedProgram {
        self.compiled()
            .satisfy(&self.witness_values())
            .expect("example program should be satisfied")
    }

    pub fn lock_time(self) -> elements::LockTime {
        elements::LockTime::from_consensus(self.lock_time)
    }

    pub fn sequence(self) -> elements::Sequence {
        elements::Sequence::from_consensus(self.sequence)
    }
}

/// Names must be unique because they serve as primary keys.
const EXAMPLES: [(&str, Example); 0] = [];

/// Iterate over the example names.
pub fn keys() -> impl ExactSizeIterator<Item = &'static str> {
    EXAMPLES.into_iter().map(|(name, _)| name)
}

/// Get the example of the given `name`.
pub fn get(name: &str) -> Option<Example> {
    EXAMPLES
        .into_iter()
        .find_map(|(found_name, found_example)| match found_name == name {
            true => Some(found_example),
            false => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn name_primary_key() {
        assert_eq!(keys().len(), keys().collect::<HashSet<_>>().len());
    }
}
