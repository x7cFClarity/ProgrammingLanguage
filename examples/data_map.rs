use x7cf_cty_pl_lib::util::data_map;
use x7cf_cty_pl_lib::util::data_map::DataMap;

enum StudentGrade {
    High, Medium, Low
}

struct Student {
    grade: StudentGrade,
    age: u8
}

fn init_preset_database(mut database: data_map::z0::DataMap<Student>) -> data_map::z0::DataMap<Student> {
    let preset = [
        ("Rayyan Khan", Student {
            age: 16,
            grade: StudentGrade::Medium
        }),
        ("Chase Barve", Student {
            age: 15,
            grade: StudentGrade::High
        }),
        ("John Doe", Student {
            age: 23,
            grade: StudentGrade::Low
        }),
        // Add 20 more students
        ("Alice Smith", Student {
            age: 17,
            grade: StudentGrade::High
        }),
        ("Bob Johnson", Student {
            age: 18,
            grade: StudentGrade::Medium
        }),
        ("Eva Davis", Student {
            age: 16,
            grade: StudentGrade::Low
        }),
        ("David Lee", Student {
            age: 19,
            grade: StudentGrade::High
        }),
        ("Grace Wilson", Student {
            age: 17,
            grade: StudentGrade::Medium
        }),
        ("Samuel Brown", Student {
            age: 20,
            grade: StudentGrade::Low
        }),
        ("Sophia White", Student {
            age: 18,
            grade: StudentGrade::High
        }),
        ("Oliver Harris", Student {
            age: 16,
            grade: StudentGrade::Medium
        }),
        ("Emma Turner", Student {
            age: 19,
            grade: StudentGrade::Low
        }),
        ("Daniel Miller", Student {
            age: 17,
            grade: StudentGrade::High
        }),
        ("Ava Moore", Student {
            age: 18,
            grade: StudentGrade::Medium
        }),
        ("William Clark", Student {
            age: 16,
            grade: StudentGrade::Low
        }),
        ("Mia Hall", Student {
            age: 19,
            grade: StudentGrade::High
        }),
        ("James Allen", Student {
            age: 17,
            grade: StudentGrade::Medium
        }),
        ("Ella Adams", Student {
            age: 20,
            grade: StudentGrade::Low
        }),
        ("Benjamin Turner", Student {
            age: 18,
            grade: StudentGrade::High
        }),
        ("Lily Davis", Student {
            age: 16,
            grade: StudentGrade::Medium
        }),
        ("Henry Smith", Student {
            age: 19,
            grade: StudentGrade::Low
        }),
    ];

    for student_tuple in preset {
        database.set_record(student_tuple.0, student_tuple.1);
    }

    database
}

fn main() {
    let mut student_database: data_map::z0::DataMap<Student> = data_map::z0::DataMap::default();
    student_database = init_preset_database(student_database);

    let rk = student_database.get_record("Rayyan Khan").as_ref().unwrap();
    println!("Initialized database contents RKxG: {}", rk.age);
}
