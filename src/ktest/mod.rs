// Copyright (c) 2025 Aeybel Varghese
//
// Quokka Testing Framework
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::mem::MaybeUninit;

/// Trait to be implemented by all test cases
pub trait KernelTest {
    fn run(&self) -> KernelTestResult;
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone)]
/// Results of a single test case
///
/// * `name`: Name of the test case
/// * `passed`: Whether it passed
/// * `message`: Error/Or Pass message if any
pub struct KernelTestResult {
    pub name: &'static str,
    pub passed: bool,
    pub message: Option<&'static str>,
}

impl KernelTestResult {
    /// Passes the current test
    ///
    /// * `name`: name of the test case
    pub fn pass(name: &'static str) -> Self {
        Self {
            name,
            passed: true,
            message: None,
        }
    }

    /// Fails the current test
    ///
    /// * `name`: name of the test case
    /// * `message`: Failure message
    pub fn fail(name: &'static str, message: &'static str) -> Self {
        Self {
            name,
            passed: false,
            message: Some(message),
        }
    }
}

/// A registry to store all test results
///
/// * `results`: Stored test results
/// * `count`: Number of stored results
pub struct KernelTestRegistry {
    results: [MaybeUninit<KernelTestResult>; 64], // TODO: [KTest] At the moment the number of possible kernel test results is a fixed number of 64, this should be configurable
    count: usize,
}

impl KernelTestRegistry {
    /// Creates a KernelTestRegistry
    pub const fn new() -> Self {
        const UNINIT: MaybeUninit<KernelTestResult> = MaybeUninit::uninit(); // TODO: [KTest] Maybe there is a better alternative to MaybeUninit?
        Self {
            results: [UNINIT; 64],
            count: 0,
        }
    }

    /// Registers a test result in the registry
    ///
    /// * `result`: Test result to record
    pub fn register_result(&mut self, result: KernelTestResult) {
        // Appends the result if registry has space
        if self.count < self.results.len() {
            self.results[self.count].write(result);
            self.count += 1;
        }
    }

    /// Returns all the Kernel Test Results
    pub fn get_results(&self) -> &[KernelTestResult] {
        // SAFETY: Only `count` items have been initialized.
        // TODO: [KTest] A better alternative has to exist to avoid the unsafe block
        unsafe {
            core::slice::from_raw_parts(
                self.results.as_ptr() as *const KernelTestResult,
                self.count,
            )
        }
    }

    /// Returns number of passed tests
    pub fn num_passed(&self) -> usize {
        self.get_results().iter().filter(|r| r.passed).count()
    }

    /// Returns the number of failed tests
    pub fn num_failed(&self) -> usize {
        self.get_results().iter().filter(|r| !r.passed).count()
    }
}

/// Runs all the tests provided in the array 'tests', and records the results in the registry
///
/// * `tests`: Lists of tests to run
/// * `registry`: Registry to store results
pub fn run_all_ktests(tests: &[&dyn KernelTest], registry: &mut KernelTestRegistry) {
    for test in tests {
        let result = test.run();
        registry.register_result(result);
    }
}
