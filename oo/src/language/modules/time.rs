use crate::language::analyzing::analyzer::{Analyzer, FunctionSignature};
use crate::language::analyzing::types::VariableType;
use crate::language::running::interpreter::{Interpreter, RuntimeFunction};
use crate::language::running::value::Value;

impl Analyzer {
    pub fn register_time_functions(&mut self) {
        
        self.functions.insert(
            vec!["Time".to_string(), "sleep".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::INT],
                return_type: VariableType::NULL,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "epoch".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "year".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "month".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "day".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "date".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::STRING,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "hour".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "minute".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "second".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "millisecond".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::INT,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "utc".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::STRING,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "now".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::STRING,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "short".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::STRING,
                called: true,
            },
        );

        self.functions.insert(
            vec!["Time".to_string(), "long".to_string()],
            FunctionSignature {
                parameters: vec![],
                return_type: VariableType::STRING,
                called: true,
            },
        );
    }

}

impl Interpreter {

    pub fn register_time_functions(&mut self) {

        self.functions.insert(
            vec!["Time".to_string(), "sleep".to_string()],
            RuntimeFunction::Native(Self::sleep),
        );

        self.functions.insert(
            vec!["Time".to_string(), "epoch".to_string()],
            RuntimeFunction::Native(Self::epoch),
        );

        self.functions.insert(
            vec!["Time".to_string(), "year".to_string()],
            RuntimeFunction::Native(Self::year),
        );

        self.functions.insert(
            vec!["Time".to_string(), "month".to_string()],
            RuntimeFunction::Native(Self::month),
        );

        self.functions.insert(
            vec!["Time".to_string(), "day".to_string()],
            RuntimeFunction::Native(Self::day),
        );

        self.functions.insert(
            vec!["Time".to_string(), "date".to_string()],
            RuntimeFunction::Native(Self::date),
        );

        self.functions.insert(
            vec!["Time".to_string(), "hour".to_string()],
            RuntimeFunction::Native(Self::hour),
        );

        self.functions.insert(
            vec!["Time".to_string(), "minute".to_string()],
            RuntimeFunction::Native(Self::minute),
        );

        self.functions.insert(
            vec!["Time".to_string(), "second".to_string()],
            RuntimeFunction::Native(Self::second),
        );

        self.functions.insert(
            vec!["Time".to_string(), "millisecond".to_string()],
            RuntimeFunction::Native(Self::millisecond),
        );

        self.functions.insert(
            vec!["Time".to_string(), "utc".to_string()],
            RuntimeFunction::Native(Self::utc),
        );

        self.functions.insert(
            vec!["Time".to_string(), "now".to_string()],
            RuntimeFunction::Native(Self::now),
        );

        self.functions.insert(
            vec!["Time".to_string(), "short".to_string()],
            RuntimeFunction::Native(Self::short),
        );

        self.functions.insert(
            vec!["Time".to_string(), "long".to_string()],
            RuntimeFunction::Native(Self::long),
        );
    }
}

impl Interpreter {

    fn sleep(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 1 {
            return Err("Time.sleep expects exactly one argument".to_string());
        }

        use std::{thread, time::Duration};

        let wait = match args[0] {
            Value::INT(i) => i,
            _ => return Err("Time.sleep expects an int".to_string()),
        };

        thread::sleep(Duration::from_secs(wait as u64));
        Ok(Value::NULL)
    }

    fn epoch(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.epoch expects no arguments".to_string());
        }

        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();

        let duration_since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!");

        let epoch = duration_since_epoch.as_secs() as i32;
        Ok(Value::INT(epoch))
    }

    fn year(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.year expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let year = date.year();
        Ok(Value::INT(year))
    }

    fn month(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.month expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let month = date.month() as i32;
        Ok(Value::INT(month))
    }

    fn day(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.day expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let day = date.day() as i32;
        Ok(Value::INT(day))
    }

    fn date(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.date expects no arguments".to_string());
        }
        
        use chrono::prelude::*;
        let time: DateTime<Local> = Local::now();
        let day = time.day() as i32;
        let month = time.month() as i32;
        let year = time.year() as i32;

        
        let date: String = format!("{}/{}/{}", month, day, year);

        Ok(Value::STRING(date))
    }

    fn hour(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.hour expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let hour = date.hour() as i32;
        Ok(Value::INT(hour))
    }

    fn minute(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.minute expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let minute = date.minute() as i32;
        Ok(Value::INT(minute))
    }

    fn second(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.second expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let second = date.second() as i32;
        Ok(Value::INT(second))
    }

    fn millisecond(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.millisecond expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let millisecond = date.timestamp_subsec_millis() as i32;
        Ok(Value::INT(millisecond))
    }

    fn utc(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.now expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Utc::now();
        let hour = date.hour() as i32;
        let minute = date.minute() as i32;
        let second = date.second() as i32;
        let time = format!("{:02}:{:02}:{:02}", hour, minute, second);
        Ok(Value::STRING(time))
    }

    fn now(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.now expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let hour = date.hour() as i32;
        let minute = date.minute() as i32;
        let second = date.second() as i32;
        let time = format!("{:02}:{:02}:{:02}", hour, minute, second);
        Ok(Value::STRING(time))
    }

    fn short(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.short expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let hour = date.hour() as i32;
        let minute = date.minute() as i32;
        let time = format!("{:02}:{:02}", hour, minute);
        Ok(Value::STRING(time))
    }

    fn long(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 0 {
            return Err("Time.long expects no arguments".to_string());
        }

        use chrono::prelude::*;
        let date = Local::now();
        let hour = date.hour() as i32;
        let minute = date.minute() as i32;
        let second = date.second() as i32;
        let millisecond = date.timestamp_subsec_millis() as i32;
        let time = format!("{:02}:{:02}:{:02}.{:03}", hour, minute, second, millisecond);
        Ok(Value::STRING(time))
    }

}