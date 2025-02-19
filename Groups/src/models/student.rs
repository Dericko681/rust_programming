use crate::{data_c::DataCollection, traits::{collect::Collect, gen_data_id::GenDataId}};
use cli_table::{format::Justify, Table};

#[derive(Debug, Clone, Table)]
pub struct Student {
     #[table(title = "ID", justify = "Justify::Right")]
     id: u32,
      #[table(title = "Name")]
    name: String,
}

impl Student {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::from(""),
        }
    }
    
}

impl Collect for Student {
    fn collect() -> Self {
        let mut student = Self::new();
       
        student.name = DataCollection::input("Enter Student's name ");

        student
    }
}
impl GenDataId<u32> for Student{
    fn set_id(&mut self, id: u32) {
        self.id = id
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
