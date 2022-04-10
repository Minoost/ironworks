use std::collections::HashMap;

/// Schema and metadata for a sheet within an Excel database.
#[derive(Debug)]
pub struct Sheet {
	/// Expected ordering of column definitions to be used when reading this schema.
	pub order: Order,

	/// The schema for the sheet.
	pub schema: Node,
}

/// Ordering of column definitions.
#[derive(Debug)]
pub enum Order {
	/// Ordered by index of definition within Excel header file.
	Index,
	/// Ordered by byte offset of columns within data.
	Offset,
}

/// Node within a sheet schema.
#[derive(Debug)]
pub enum Node {
	/// An array of two or more sub-schemas.
	#[allow(missing_docs)]
	Array { count: u32, schema: Box<Node> },

	// TODO: Reference fields
	/// A reference to one or more rows in other sheets.
	Reference,

	/// A single scalar field with no further semantics.
	Scalar,

	/// A collection of named sub-schemas.
	Struct(HashMap<String, Node>),
}
