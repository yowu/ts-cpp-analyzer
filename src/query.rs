use tree_sitter::Query;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SymbolType {
    Class,
    Field,
    FieldAccess,
    Function,
    FunctionCall,
    MethodCall,
    Parameter,
    Type,
    Variable,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QueryPattern {
    Class(&'static str),
    Field(&'static str),
    FieldAccess(&'static str),
    Function(&'static str),
    FunctionCall(&'static str),
    MethodCall(&'static str),
    Parameter(&'static str),
    Type(&'static str),
    Variable(&'static str),
}

impl QueryPattern {
    pub fn pattern(&self) -> &'static str {
        match self {
            QueryPattern::Class(p) => p,
            QueryPattern::Field(p) => p,
            QueryPattern::FieldAccess(p) => p,
            QueryPattern::Function(p) => p,
            QueryPattern::FunctionCall(p) => p,
            QueryPattern::MethodCall(p) => p,
            QueryPattern::Parameter(p) => p,
            QueryPattern::Type(p) => p,
            QueryPattern::Variable(p) => p,
        }
    }

    pub fn symbol_type(&self) -> SymbolType {
        match self {
            QueryPattern::Class(_) => SymbolType::Class,
            QueryPattern::Field(_) => SymbolType::Field,
            QueryPattern::FieldAccess(_) => SymbolType::FieldAccess,
            QueryPattern::Function(_) => SymbolType::Function,
            QueryPattern::FunctionCall(_) => SymbolType::FunctionCall,
            QueryPattern::MethodCall(_) => SymbolType::MethodCall,
            QueryPattern::Parameter(_) => SymbolType::Parameter,
            QueryPattern::Type(_) => SymbolType::Type,
            QueryPattern::Variable(_) => SymbolType::Variable,
        }
    }
}

#[derive(Debug)]
pub struct PreparedQuery {
    pub query: Query,
    query_pattern: QueryPattern,
}

impl PreparedQuery {
    pub fn new(
        language: &tree_sitter::Language,
        query_pattern: QueryPattern,
    ) -> Result<Self, String> {
        let query = Query::new(language, &query_pattern.pattern())
            .map_err(|e| format!("Failed to compile query: {}", e))?;
        Ok(Self {
            query,
            query_pattern,
        })
    }

    pub fn symbol_type(&self) -> SymbolType {
        self.query_pattern.symbol_type()
    }
}

pub struct QueryManager {
    pub name: String,
    pub language: tree_sitter::Language,
    prepared_queries: Vec<PreparedQuery>,
}

impl QueryManager {
    /// Validates the entire configuration
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Configuration name cannot be empty".to_string());
        }

        if self.prepared_queries.is_empty() {
            return Err("At least one query pattern must be defined".to_string());
        }
        Ok(())
    }

    fn prepare_queries(&mut self, query_patterns: Vec<QueryPattern>) -> Result<(), String> {
        self.prepared_queries.clear();
        for pattern in query_patterns {
            let compiled = PreparedQuery::new(&self.language, pattern)?;
            self.prepared_queries.push(compiled);
        }
        Ok(())
    }

    pub fn get_queries(&self) -> &[PreparedQuery] {
        &self.prepared_queries
    }
}

pub struct QueryManagerBuilder {
    name: String,
    language: Option<tree_sitter::Language>,
    queries: Vec<QueryPattern>,
}

impl QueryManagerBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            language: None,
            queries: Vec::new(),
        }
    }

    pub fn with_language(mut self, language: tree_sitter::Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn with_queries(mut self, queries: Vec<QueryPattern>) -> Self {
        self.queries = queries;
        self
    }

    pub fn add_query(mut self, query: QueryPattern) -> Self {
        self.queries.push(query);
        self
    }

    pub fn build(self) -> Result<QueryManager, String> {
        let language = self.language.ok_or("Language not set")?;
        let mut manager = QueryManager {
            name: self.name,
            language,
            prepared_queries: Vec::new(),
        };
        manager.prepare_queries(self.queries)?;
        manager.validate()?;
        Ok(manager)
    }
}
