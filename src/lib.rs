
mod task;
mod task_generated;
use std::mem::size_of_val;
pub use task_generated::medo::copiyer::{CopyModifier, Task, Filter, TaskArgs, FilterArgs};

pub fn flatbuffer_serialize()
{
    // Build up a serialized buffer algorithmically.
    // Initialize it with a capacity of 1024 bytes.
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let name = builder.create_string("Имя задачи");
    let description = builder.create_string("тестовая задача созданная с помощью билдера");
    let source_dir = builder.create_string("директория откуда производится копирование");
    let target_dir = builder.create_string("директория куда производится копирование");
    let timer: u64 = 15000;
    let delete_after_copy = false;
    let copy_modifier =  CopyModifier::CopyAll;
    let is_active =  true;
    let clean_type_val = builder.create_string("Квитанция");
    let clean_types = builder.create_vector(&[clean_type_val]);
    let generate_exclude_file = false;
    let color = builder.create_string("#45e4");

    let doc_type_filter = builder.create_string("тестовый вид документа для фильтрации");
    let doc_types_filter = builder.create_vector(&[doc_type_filter]);
    let doc_uid_filter = builder.create_string("тестовый uid документа для фильтрации");
    let doc_uid_filter = builder.create_vector(&[doc_uid_filter]);

    let filter = Filter::create(&mut builder, &FilterArgs
    {
        document_types: Some(doc_types_filter),
        document_uids: Some(doc_uid_filter)
    });
    let task = Task::create(&mut builder, &TaskArgs 
    {
        name: Some(name),
        description: Some(description),
        source_dir: Some(source_dir),
        target_dir: Some(target_dir),
        timer: timer as i64,
        delete_after_copy,
        copy_modifier,
        is_active,
        clean_types : Some(clean_types),
        generate_exclude_file,
        color: Some(color),
        filters: Some(filter)
    });
    
    // Serialize the root of the object, without providing a file identifier.
    builder.finish(task, None);

    // We now have a FlatBuffer we can store on disk or send over a network.
    // ** file/network code goes here :) **
    // Instead, we're going to access it right away (as if we just received it).
    // This must be called after `finish()`.
    let buf = builder.finished_data(); // Of type `&[u8]`
    let size = size_of_val(buf);
    println!("Task занимает {} байт.", size);
    // Get access to the root:
    let task = flatbuffers::root::<Task>(buf).unwrap();

    // Get and test some scalar types from the FlatBuffer.
    let name = task.name();
    let descr = task.description();
    let filter = task.filters().unwrap();
    assert_eq!(name, "Имя задачи");
    assert_eq!(descr, "тестовая задача созданная с помощью билдера");
    assert_eq!(filter.document_types().get(0), "тестовый вид документа для фильтрации")

}

#[cfg(test)]
mod tests 
{
    use std::path::PathBuf;
    use serde::{Deserialize, Serialize};
    pub use crate::task::*;
    pub use super::flatbuffer_serialize;

    #[test]
    fn test_flexbuffer_serialize() 
    {
        let f =  Filter 
        { 
            document_types : vec!["тестовый вид документа для фильтрации".to_owned()],
            document_uids: vec!["тестовый uid документа для фильтрации".to_owned()] 
        };
        let task = Task
        {
                name: "Имя задачи".to_owned(),
                description: "тестовая задача созданная с помощью сериализатора".to_owned(),
                source_dir: PathBuf::from("директория откуда производится копирование"),
                target_dir: PathBuf::from("директория куда производится копирование"),
                timer: 15000,
                delete_after_copy: false,
                copy_modifier: CopyModifier::CopyAll,
                is_active: true,
                clean_types: vec!["Квитанция".to_owned()],
                generate_exclude_file: false,
                color: "#45e4".to_owned(),
                filters: f
        };

        let mut s = flexbuffers::FlexbufferSerializer::new();
        task.serialize(&mut s).unwrap();
        let r = flexbuffers::Reader::get_root(s.view()).unwrap();
        println!("Task занимает {:?} байт.", s.view().len());
        println!("{}", r);
        let task2 = Task::deserialize(r).unwrap();
        assert_eq!(task, task2);
    }

    #[test]
    fn test_flatbuffer_serialize()
    {
        
        flatbuffer_serialize();
    }
}
