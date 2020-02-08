struct Person { job: Option<Job> }

#[derive(Copy, Clone)]
struct Job { phone_number: Option<PhoneNumber> }

#[derive(Copy, Clone)]
struct PhoneNumber { area_code: Option<u8>, number: u32 }

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_2_1_question_mark() {
        let person = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    number: 439222222,
                })
            })
        };

        assert_eq!(person.work_phone_area_code(), Some(61));
    }
}