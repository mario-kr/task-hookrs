use std::result::Result as RResult;

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;
use serde::de::Error as SerdeError;
use serde::ser::MapVisitor;
use serde::de::MapVisitor as DeserializeMapVisitor;
use uuid::Uuid;

use priority::TaskPriority;
use status::TaskStatus;
use project::Project;
use tag::Tag;
use date::Date;
use annotation::Annotation;

#[derive(Debug, Clone)]
pub struct Task {
    status      : TaskStatus,
    uuid        : Uuid,
    entry       : Date,
    description : String,

    annotations : Option<Vec<Annotation>>,
    depends     : Option<String>,
    due         : Option<Date>,
    end         : Option<Date>,
    imask       : Option<i64>,
    mask        : Option<String>,
    modified    : Option<Date>,
    parent      : Option<Uuid>,
    priority    : Option<TaskPriority>,
    project     : Option<Project>,
    recur       : Option<String>,
    scheduled   : Option<Date>,
    start       : Option<Date>,
    tags        : Option<Vec<Tag>>,
    until       : Option<Date>,
    wait        : Option<Date>,
}

/*
 * TODO: We do not fail if the JSON parsing fails. This panics. We rely on taskwarrior to be nice
 * to us. I guess this should be fixed.
 */
impl Task {

    pub fn new(
        status      : TaskStatus,
        uuid        : Uuid,
        entry       : Date,
        description : String,

        annotations : Option<Vec<Annotation>>,
        depends     : Option<String>,
        due         : Option<Date>,
        end         : Option<Date>,
        imask       : Option<i64>,
        mask        : Option<String>,
        modified    : Option<Date>,
        parent      : Option<Uuid>,
        priority    : Option<TaskPriority>,
        project     : Option<Project>,
        recur       : Option<String>,
        scheduled   : Option<Date>,
        start       : Option<Date>,
        tags        : Option<Vec<Tag>>,
        until       : Option<Date>,
        wait        : Option<Date>,
    ) -> Task
    {
        Task {
            status      : status,
            uuid        : uuid,
            entry       : entry,
            description : description,

            annotations : annotations,
            depends     : depends,
            due         : due,
            end         : end,
            imask       : imask,
            mask        : mask,
            modified    : modified,
            parent      : parent,
            priority    : priority,
            project     : project,
            recur       : recur,
            scheduled   : scheduled,
            start       : start,
            tags        : tags,
            until       : until,
            wait        : wait,
        }
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn entry(&self) -> &Date {
        &self.entry
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn annotations(&self) -> Option<&Vec<Annotation>> {
        self.annotations.as_ref()
    }

    pub fn depends(&self) -> Option<&String> {
        self.depends.as_ref()
    }

    pub fn due(&self) -> Option<&Date> {
        self.due.as_ref()
    }

    pub fn end(&self) -> Option<&Date> {
        self.end.as_ref()
    }

    pub fn imask(&self) -> Option<&i64> {
        self.imask.as_ref()
    }

    pub fn mask(&self) -> Option<&String> {
        self.mask.as_ref()
    }

    pub fn modified(&self) -> Option<&Date> {
        self.modified.as_ref()
    }

    pub fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref()
    }

    pub fn priority(&self) -> Option<&TaskPriority> {
        self.priority.as_ref()
    }

    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    pub fn recur(&self) -> Option<&String> {
        self.recur.as_ref()
    }

    pub fn scheduled(&self) -> Option<&Date> {
        self.scheduled.as_ref()
    }

    pub fn start(&self) -> Option<&Date> {
        self.start.as_ref()
    }

    pub fn tags(&self) -> Option<&Vec<Tag>> {
        self.tags.as_ref()
    }

    pub fn until(&self) -> Option<&Date> {
        self.until.as_ref()
    }

    pub fn wait(&self) -> Option<&Date> {
        self.wait.as_ref()
    }

}

impl Serialize for Task {

    fn serialize<S>(&self, serializer: &mut S) -> RResult<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_struct("Task", TaskVisitor {
            value: self,
            state: 0,
        })
    }

}

struct TaskVisitor<'a> {
    value: &'a Task,
    state: u8,
}

impl<'a> MapVisitor for TaskVisitor<'a> {

    fn visit<S>(&mut self, serializer: &mut S) -> RResult<Option<()>, S::Error>
        where S: Serializer
    {
        match self.state {
            0 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("status", &self.value.status))))
            },
            1 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("uuid", &self.value.uuid))))
            },
            2 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("entry", &self.value.entry))))
            },
            3 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("description", &self.value.description))))
            },
            4 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("annotations", &self.value.annotations))))
            },
            5 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("depends", &self.value.depends))))
            },
            6 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("due", &self.value.due))))
            },
            7 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("end", &self.value.end))))
            },
            8 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("imask", &self.value.imask))))
            },
            9 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("mask", &self.value.mask))))
            },
            10 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("modified", &self.value.modified))))
            },
            11 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("parent", &self.value.parent))))
            },
            12 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("priority", &self.value.priority))))
            },
            13 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("project", &self.value.project))))
            },
            14 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("recur", &self.value.recur))))
            },
            15 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("scheduled", &self.value.scheduled))))
            },
            16 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("start", &self.value.start))))
            },
            17 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("tags", &self.value.tags))))
            },
            18 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("until", &self.value.until))))
            },
            19 => {
                self.state += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("wait", &self.value.wait))))
            },
            _ => {
                Ok(None)
            }
        }
    }

}

impl Deserialize for Task {

    fn deserialize<D>(deserializer: &mut D) -> RResult<Task, D::Error>
        where D: Deserializer
    {
        static FIELDS: &'static [&'static str] = &[
            "status",
            "uuid",
            "entry",
            "description",

            "annotations",
            "depends",
            "due",
            "end",
            "imask",
            "mask",
            "modified",
            "parent",
            "priority",
            "project",
            "recur",
            "scheduled",
            "start",
            "tags",
            "until",
            "wait"
        ];
        deserializer.deserialize_struct("Task", FIELDS, TaskDeserializeVisitor)
    }

}

struct TaskDeserializeVisitor;

impl Visitor for TaskDeserializeVisitor {
    type Value = Task;

    fn visit_map<V>(&mut self, mut visitor: V) -> RResult<Task, V::Error>
        where V: DeserializeMapVisitor
    {
        let mut status      = None;
        let mut uuid        = None;
        let mut entry       = None;
        let mut description = None;

        let mut annotations = None;
        let mut depends     = None;
        let mut due         = None;
        let mut end         = None;
        let mut imask       = None;
        let mut mask        = None;
        let mut modified    = None;
        let mut parent      = None;
        let mut priority    = None;
        let mut project     = None;
        let mut recur       = None;
        let mut scheduled   = None;
        let mut start       = None;
        let mut tags        = None;
        let mut until       = None;
        let mut wait        = None;

        loop {
            let key : Option<String> = try!(visitor.visit_key());
            if key.is_none() {
                break;
            }
            let key = key.unwrap();

            match &key[..] {
                "status" => {
                    status = Some(try!(visitor.visit_value()));
                },
                "uuid" => {
                    uuid = Some(try!(visitor.visit_value()));
                },
                "entry" => {
                    entry = Some(try!(visitor.visit_value()));
                },
                "description" => {
                    description = Some(try!(visitor.visit_value()));
                },

                "annotations" => {
                    annotations = Some(try!(visitor.visit_value()));
                },
                "depends" => {
                    depends = Some(try!(visitor.visit_value()));
                },
                "due" => {
                    due = Some(try!(visitor.visit_value()));
                },
                "end" => {
                    end = Some(try!(visitor.visit_value()));
                },
                "imask" => {
                    imask = Some(try!(visitor.visit_value()));
                },
                "mask" => {
                    mask = Some(try!(visitor.visit_value()));
                },
                "modified" => {
                    modified = Some(try!(visitor.visit_value()));
                },
                "parent" => {
                    parent = Some(try!(visitor.visit_value()));
                },
                "priority" => {
                    priority = Some(try!(visitor.visit_value()));
                },
                "project" => {
                    project = Some(try!(visitor.visit_value()));
                },
                "recur" => {
                    recur = Some(try!(visitor.visit_value()));
                },
                "scheduled" => {
                    scheduled = Some(try!(visitor.visit_value()));
                },
                "start" => {
                    start = Some(try!(visitor.visit_value()));
                },
                "tags" => {
                    tags = Some(try!(visitor.visit_value()));
                },
                "until" => {
                    until = Some(try!(visitor.visit_value()));
                },
                "wait" => {
                    wait = Some(try!(visitor.visit_value()));
                },

                field => {
                    use serde::de::impls::IgnoredAny;

                    debug!("field '{}' ignored", field);
                    let _: IgnoredAny = try!(visitor.visit_value());
                }
            }
        }

        let status = match status {
            Some(status) => status,
            None => try!(visitor.missing_field("status")),
        };

        let uuid = match uuid {
            Some(uuid) => uuid,
            None => try!(visitor.missing_field("uuid")),
        };

        let entry = match entry {
            Some(entry) => entry,
            None => try!(visitor.missing_field("entry")),
        };

        let description = match description {
            Some(description) => description,
            None => try!(visitor.missing_field("description")),
        };

        try!(visitor.end());

        let task = Task::new(
            status,
            uuid,
            entry,
            description,

            annotations,
            depends,
            due,
            end,
            imask,
            mask,
            modified,
            parent,
            priority,
            project,
            recur,
            scheduled,
            start,
            tags,
            until,
            wait
        );

        Ok(task)
    }
}

#[cfg(test)]
mod test {
    use date::Date;
    use date::TASKWARRIOR_DATETIME_TEMPLATE;
    use status::TaskStatus;
    use task::Task;

    use uuid::Uuid;
    use chrono::naive::datetime::NaiveDateTime;
    use serde_json;

    fn mkdate(s: &str) -> Date {
        let n = NaiveDateTime::parse_from_str(s, TASKWARRIOR_DATETIME_TEMPLATE);
        Date::from(n.unwrap())
    }

    #[test]
    fn test_deser() {
        let s =
r#"{
"id": 1,
"description": "test",
"entry": "20150619T165438Z",
"status": "waiting",
"uuid": "8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task : Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "test");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap());

        let back = serde_json::to_string(&task).unwrap();

        assert!(back.contains("description"));
        assert!(back.contains("test"));
        assert!(back.contains("entry"));
        assert!(back.contains("20150619T165438Z"));
        assert!(back.contains("status"));
        assert!(back.contains("waiting"));
        assert!(back.contains("uuid"));
        assert!(back.contains("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"));
    }

    #[test]
    fn test_deser_more() {
        let s =
r#"{
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
}"#;

        println!("{}", s);

        let task = serde_json::from_str(s);
        println!("{:?}", task);
        assert!(task.is_ok());
        let task : Task = task.unwrap();

        assert!(task.status().clone() == TaskStatus::Waiting);
        assert!(task.description() == "some description");
        assert!(task.entry().clone() == mkdate("20150619T165438Z"));
        assert!(task.uuid().clone() == Uuid::parse_str("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0").unwrap());

        assert!(task.modified() == Some(&mkdate("20160327T164007Z")));
        assert!(task.project() == Some(&String::from("someproject")));

        if let Some(tags) = task.tags() {
            for tag in tags {
                let any_tag = [ "some", "tags", "are", "here", ]
                    .into_iter().any(|t| tag == *t);
                assert!(any_tag, "Tag {} missing", tag);
            }
        } else {
                assert!(false, "Tags completely missing");
        }

        assert!(task.wait() == Some(&mkdate("20160508T164007Z")));
        // assert!(task.urgency().clone() == 0.583562);

        let back = serde_json::to_string(&task).unwrap();

        assert!(back.contains("description"));
        assert!(back.contains("some description"));
        assert!(back.contains("entry"));
        assert!(back.contains("20150619T165438Z"));
        assert!(back.contains("project"));
        assert!(back.contains("someproject"));
        assert!(back.contains("status"));
        assert!(back.contains("waiting"));
        assert!(back.contains("tags"));
        assert!(back.contains("some"));
        assert!(back.contains("tags"));
        assert!(back.contains("are"));
        assert!(back.contains("here"));
        assert!(back.contains("uuid"));
        assert!(back.contains("8ca953d5-18b4-4eb9-bd56-18f2e5b752f0"));
    }


}

