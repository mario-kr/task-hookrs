//! Module containing the `import()` function

use std::io::BufRead;
use std::io::Read;

use serde_json;

use result::Result;
use task::Task;
use error::{TaskError, TaskErrorKind};

/// Import taskwarrior-exported JSON. This expects an JSON Array of objects, as exported by
/// taskwarrior.
pub fn import<R: Read>(r: R) -> Result<Vec<Task>> {
    serde_json::from_reader(r)
        .map_err(|e| {
            TaskError::new(TaskErrorKind::ParserError, Some(Box::new(e)))
        })
}

/// Import a single JSON-formatted Task
pub fn import_task(s : &str) -> Result<Task> {
    serde_json::from_str(s)
        .map_err(|e| {
            TaskError::new(TaskErrorKind::ParserError, Some(Box::new(e)))
        })
}

/// Reads line by line and tries to parse a task-object per line.
pub fn import_tasks<BR: BufRead>(r : BR) -> Vec<Result<Task>> {
    let mut vt = Vec::new();
    for line in r.lines() {
        if line.is_err() {
            vt.push(Err(TaskError::new(TaskErrorKind::ReaderError, Some(Box::new(line.unwrap_err())))));
            continue;
        }
        // Unwrap is safe because of continue above
        if line.as_ref().unwrap().len() <= 0 {
            // Empty strings are not usable, and shall be silently ignored
            continue;
        }
        vt.push(import_task(line.unwrap().as_str()));
    }
    vt
}

#[test]
fn test_one() {
    let s = r#"
[
    {
        "id": 1,
        "description": "some description",
        "entry": "20150619T165438Z",
        "modified": "20160327T164007Z",
        "project": "someproject",
        "status": "waiting",
        "tags": ["some", "tags", "are", "here"],
        "uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
        "wait": "20160508T164007Z",
        "urgency": 0.583562
    }
]
"#;

    let imported = import(s.as_bytes());
    assert!(imported.is_ok());
    let imported = imported.unwrap();
    assert!(imported.len() == 1);
}

#[test]
fn test_two() {
    let s = r#"
[
    {
        "id"          : 1,
        "description" : "test",
        "entry"       : "20150619T165438Z",
        "modified"    : "20160327T164007Z",
        "project"     : "self.software",
        "status"      : "waiting",
        "tags"        : ["check", "this", "crate", "out"],
        "uuid"        : "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
        "wait"        : "20160508T164007Z",
        "urgency"     : 0.583562
    },
    {
        "id"          : 2,
        "description" : "another test",
        "entry"       : "20150623T181011Z",
        "modified"    : "20160327T163718Z",
        "priority"    : "L",
        "project"     : "studying",
        "status"      : "waiting",
        "tags"        : ["I", "have", "awesome", "kittens"],
        "uuid"        : "54d49ffc-a06b-4dd8-b7d1-db5f50594312",
        "wait"        : "20160508T163718Z",
        "annotations" : [
            {
                "entry"       : "20150623T181018Z",
                "description" : "fooooooobar"
            }
        ],
        "urgency"     : 3.16164
    },
    {
        "id"          : 3,
        "description" : "I love kittens, really!",
        "entry"       : "20150919T222323Z",
        "modified"    : "20160327T163718Z",
        "project"     : "getkittens",
        "status"      : "waiting",
        "tags"        : ["kittens", "are", "so", "damn", "awesome"],
        "uuid"        : "08ee8dce-cb97-4c8c-9940-c9a440e90119",
        "wait"        : "20160508T163718Z",
        "urgency"     : 1.07397
    }
]

"#;

    let imported = import(s.as_bytes());
    assert!(imported.is_ok());
    let imported = imported.unwrap();
    assert!(imported.len() == 3);
}

#[test]
fn test_one_single() {
    use status::TaskStatus;
    let s = r#"
{
    "id": 1,
    "description": "some description",
    "entry": "20150619T165438Z",
    "modified": "20160327T164007Z",
    "project": "someproject",
    "status": "waiting",
    "tags": ["some", "tags", "are", "here"],
    "uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0",
    "wait": "20160508T164007Z",
    "urgency": 0.583562
}
"#;
    let imported = import_task(&s);
    assert!(imported.is_ok());
    let imported = imported.unwrap();
    assert!(imported.status() == &TaskStatus::Waiting);
}

#[test]
fn test_two_single() {
    use std::io::BufReader;
    use status::TaskStatus;
    let s = r#"
{"id":1,"description":"some description","entry":"20150619T165438Z","modified":"20160327T164007Z","project":"someproject","status":"waiting","tags":["some","tags","are","here"],"uuid":"8ca953d5-18b4-4eb9-bd56-18f2e5b752f0","wait":"20160508T164007Z","urgency":0.583562}
{"id":1,"description":"some description","entry":"20150619T165438Z","modified":"20160327T164007Z","project":"someproject","status":"waiting","tags":["some","tags","are","here"],"uuid":"8ca953d5-18b4-4eb9-bd56-18f2e5b752f0","wait":"20160508T164007Z","urgency":0.583562}"#;
    let imported = import_tasks(BufReader::new(s.as_bytes()));
    assert!(imported.len() == 2);
    assert!(imported[0].is_ok());
    assert!(imported[1].is_ok());
    let import0 = imported[0].as_ref().unwrap();
    let import1 = imported[1].as_ref().unwrap();
    assert!(import0.status() == &TaskStatus::Waiting);
    assert!(import1.status() == &TaskStatus::Waiting);
}

