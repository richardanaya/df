use std::error::Error;

type DataInteger = i32;
type DataText = String;

#[derive(Debug)]
struct Column<T> {
    name: String,
    data: Vec<T>,
}

#[derive(Debug)]
pub struct DataFrame {
    columns: Vec<DataColumn>,
}

#[derive(Debug)]
enum DataColumn {
    IntegerDataColumn(Column<DataInteger>),
    TextDataColumn(Column<DataText>),
}

#[derive(Debug)]
struct DataFrameError {
    msg: String,
}

impl DataFrameError {
    fn create(msg: &str) -> Box<dyn Error> {
        Box::new(DataFrameError {
            msg: msg.to_owned(),
        })
    }
}

impl std::fmt::Display for DataFrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for DataFrameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl DataFrame {
    pub fn new(
        column_names: Vec<String>,
        data: Vec<DataCell>,
    ) -> Result<DataFrame, Box<dyn Error>> {
        let num_cols = column_names.len();
        let mut column_types = vec![];

        // Figure out the column types from the data
        for i in 0..num_cols {
            if i >= data.len() {
                // Default to integer
                column_types.push(DataTypes::Integer);
            } else {
                column_types.push(data[i].data_type());
            }
        }

        if data.len() % num_cols != 0 {
            return Err(DataFrameError::create(
                "length of data provided did not match expected number of columns",
            ));
        }

        // create columns based on column types
        let mut cols = Vec::<DataColumn>::new();
        for (i, v) in column_types.iter().enumerate() {
            match v {
                DataTypes::Integer => {
                    cols.push(DataColumn::IntegerDataColumn(Column::<DataInteger> {
                        name: column_names[i].clone(),
                        data: vec![],
                    }))
                }
                DataTypes::Text => cols.push(DataColumn::TextDataColumn(Column::<DataText> {
                    name: column_names[i].clone(),
                    data: vec![],
                })),
            }
        }

        // Go through each data cell and if they can be added to the appropriate column, do it
        for (i, cell) in data.iter().enumerate() {
            let col_index = i % num_cols;
            match &mut cols[col_index] {
                DataColumn::IntegerDataColumn(col) => match &cell {
                    DataCell::IntegerDataCell(val) => col.data.push(val.clone()),
                    _ => {
                        return Err(DataFrameError::create(
                            "data cell type did not match integer column type",
                        ))
                    }
                },
                DataColumn::TextDataColumn(col) => match &cell {
                    DataCell::TextDataCell(val) => col.data.push(val.clone()),
                    _ => {
                        return Err(DataFrameError::create(
                            "data cell type did not match text column type",
                        ))
                    }
                },
            }
        }

        Ok(DataFrame { columns: cols })
    }
}

#[derive(Debug)]
enum DataTypes {
    Integer,
    Text,
}

#[derive(Debug)]
pub enum DataCell {
    IntegerDataCell(DataInteger),
    TextDataCell(DataText),
}

impl DataCell {
    fn data_type(&self) -> DataTypes {
        match self {
            DataCell::IntegerDataCell(_) => DataTypes::Integer,
            DataCell::TextDataCell(_) => DataTypes::Text,
        }
    }
}

impl From<DataInteger> for DataCell {
    fn from(v: DataInteger) -> Self {
        DataCell::IntegerDataCell(v)
    }
}

impl From<DataText> for DataCell {
    fn from(v: DataText) -> Self {
        DataCell::TextDataCell(v)
    }
}

impl From<&str> for DataCell {
    fn from(v: &str) -> Self {
        DataCell::TextDataCell(v.to_owned())
    }
}

#[macro_export]
macro_rules! data {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::<DataCell>::new();
            $(
                temp_vec.push(DataCell::from($x));
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! columns {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::<String>::new();
            $(
                temp_vec.push($x.to_owned());
            )*
            temp_vec
        }
    };
}
