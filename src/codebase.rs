use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::{Error, SimpleFile};
use std::rc::Rc;

/// Codebase. A struct that holds all your code in memory (codespan forces this)
#[derive(Debug)]
pub struct Codebase {
    config: codespan_reporting::term::Config,
    files: Vec<SimpleFile<String, Rc<str>>>,
    errors: usize,
    warnings: usize,
}

impl Codebase {
    /// Create a new codebase.
    pub fn new() -> Self {
        Self {
            config: codespan_reporting::term::Config::default(),
            files: Vec::new(),
            errors: 0,
            warnings: 0,
        }
    }

    /// Add a file to the codebase, returning the handle that can be used to
    /// refer to it again.
    pub fn add(&mut self, name: String, source: String) -> usize {
        let file_id = self.files.len();
        self.files.push(SimpleFile::new(name, Rc::from(source)));
        file_id
    }

    /// Get the file corresponding to the given id.
    pub fn get(&self, file_id: usize) -> Result<&SimpleFile<String, Rc<str>>, Error> {
        self.files.get(file_id).ok_or(Error::FileMissing)
    }

    /// Emit a diagnostic
    pub fn emit(&mut self, diagnostic: Diagnostic<usize>) {
        match diagnostic.severity {
            codespan_reporting::diagnostic::Severity::Bug => (),
            codespan_reporting::diagnostic::Severity::Error => self.errors += 1,
            codespan_reporting::diagnostic::Severity::Warning => self.warnings += 1,
            codespan_reporting::diagnostic::Severity::Note => (),
            codespan_reporting::diagnostic::Severity::Help => (),
        };
        let mut writer = codespan_reporting::term::termcolor::StandardStream::stderr(
            codespan_reporting::term::termcolor::ColorChoice::Auto,
        );
        codespan_reporting::term::emit(&mut writer, &self.config, self, &diagnostic)
            .expect("internal error");
    }

    /// Get the number of errors emitted
    pub fn errors(&self) -> usize {
        self.errors
    }

    /// Get the number of warnings emitted
    pub fn warnings(&self) -> usize {
        self.warnings
    }

    /// Clear compilation status
    pub fn clear(&mut self) {
        self.warnings = 0;
        self.errors = 0;
    }
}

impl Default for Codebase {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> codespan_reporting::files::Files<'a> for Codebase {
    type FileId = usize;
    type Name = String;
    type Source = &'a str;

    fn name(&self, file_id: usize) -> Result<String, Error> {
        Ok(self.get(file_id)?.name().clone())
    }

    fn source(&self, file_id: usize) -> Result<&str, Error> {
        Ok(self.get(file_id)?.source().as_ref())
    }

    fn line_index(&self, file_id: usize, byte_index: usize) -> Result<usize, Error> {
        self.get(file_id)?.line_index((), byte_index)
    }

    fn line_range(
        &self,
        file_id: usize,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, Error> {
        self.get(file_id)?.line_range((), line_index)
    }
}
