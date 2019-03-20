mod core;
extern crate futures;
extern crate tokio;
extern crate xml;
use core::xmlerror::*;
use core::xmlutil::{
    characters, end_element, find_start_element, peek_at_name, skip_tree, start_element,
};
use core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use core::DeserializerNext;
use std::io::Write;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::reader::XmlEvent;
use xml::EventReader;

#[derive(Debug, Default)]
struct SummaryResult {
    pub key: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Default)]
struct Results {
    pub key: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Default)]
struct Question {
    pub prompt: Option<String>,

    pub question_type: Option<String>,

    pub answers: Vec<String>,
}

#[derive(Debug, Default)]
struct IdologyIdResponse {
    pub id: Option<String>,

    pub summary_result: Option<SummaryResult>,

    pub results: Option<Results>,

    pub questions: Option<Vec<Question>>,
}
struct IdologyIdResponseDeserializer;
impl IdologyIdResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IdologyIdResponse, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IdologyIdResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "id-number" => {
                        obj.id = Some(try!(IdologyIdNumberDeserializer::deserialize(
                            "id-number",
                            stack
                        )));
                    }
                    "summary-result" => {
                        obj.summary_result = Some(try!(
                            IdologySummaryResultDeserializer::deserialize("summary-result", stack)
                        ));
                    }
                    "results" => {
                        obj.results = Some(try!(IdologyResultsDeserializer::deserialize(
                            "results", stack
                        )));
                    }
                    "questions" => {
                        obj.questions = Some(try!(IdologyQuestionsSetDeserializer::deserialize(
                            "questions",
                            stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdologyIdNumberDeserializer;
impl IdologyIdNumberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdologySummaryResultDeserializer;
impl IdologySummaryResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SummaryResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SummaryResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "key" => {
                        obj.key = Some(try!(IdologyKeyDeserializer::deserialize("key", stack)));
                    }
                    "message" => {
                        obj.message = Some(try!(IdologyMessageDeserializer::deserialize(
                            "message", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdologyResultsDeserializer;
impl IdologyResultsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Results, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Results::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "key" => {
                        obj.key = Some(try!(IdologyKeyDeserializer::deserialize("key", stack)));
                    }
                    "message" => {
                        obj.message = Some(try!(IdologyMessageDeserializer::deserialize(
                            "message", stack
                        )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

/// key message
struct IdologyKeyDeserializer;
impl IdologyKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdologyMessageDeserializer;
impl IdologyMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdologyQuestionsSetDeserializer;
impl IdologyQuestionsSetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Question>, XmlParseError> {
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "question" {
                        obj.push(try!(IdologyQuestionDeserializer::deserialize(
                            "question", stack
                        )));
                    } else {
                        skip_tree(stack);
                    }
                }
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        Ok(obj)
    }
}
struct IdologyQuestionDeserializer;
impl IdologyQuestionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Question, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = Question::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => match &name[..] {
                    "prompt" => {
                        obj.prompt = Some(try!(IdologyPromptDeserializer::deserialize(
                            "prompt", stack
                        )));
                    }
                    "type" => {
                        obj.question_type = Some(try!(
                            IdologyQuestionTypeDeserializer::deserialize("type", stack)
                        ));
                    }
                    "answer" => {
                        obj.answers
                            .push(try!(IdologyAnswerDeserializer::deserialize(
                                "answer", stack
                            )));
                    }
                    _ => skip_tree(stack),
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => {
                    stack.next();
                }
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdologyPromptDeserializer;
impl IdologyPromptDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}
struct IdologyQuestionTypeDeserializer;
impl IdologyQuestionTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

struct IdologyAnswerDeserializer;
impl IdologyAnswerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
    }
}

fn main() {
    let s = r##"
<response>

<id-number>2299859641</id-number>

<summary-result>
<key>id.success</key>
<message>PASS</message>
</summary-result>

<results>
<key>result.match</key>
<message>ID Located</message>
</results>

<questions><question>
<prompt>At which of the following addresses have you lived?</prompt>
<type>previous.address</type>
<answer>16015 RACETRACK RD</answer>
<answer>3 CRESSING CT</answer>
<answer>510 ADAMS RD</answer>
<answer>None of the above</answer>
<answer>Skip Question</answer>

</question><question>
<prompt>In which county have you lived?</prompt>
<type>current.county.b</type>
<answer>PREBLE</answer><answer>MEDINA</answer>
<answer>FULTON</answer>
<answer>None of the above</answer>
<answer>Skip Question</answer>

</question><question>
<prompt>Between 1979 and 1980, in which State did you live?</prompt>
<type>prior.residence.state.multiyear</type>
<answer>COLORADO</answer>
<answer>NEW YORK</answer>
<answer>ALASKA</answer>
<answer>None of the above</answer>
<answer>Skip Question</answer>
</question><question>
<prompt>How long have you been associated with the property at 222333 PEACHTREE PLACE?</prompt>

<type>time.at.current.address</type>
<answer>Less than 2 years</answer>
<answer>2 - 3 years</answer>
<answer>3 - 4 years</answer>
<answer>4 - 5 years</answer>
<answer>Over 5 years</answer>
<answer>None of the above</answer>
<answer>Skip Question</answer>

</question></questions>
<id-scan>no</id-scan>

</response>
    "##;
    let mut reader = EventReader::from_str(&s);
    let mut stack = XmlResponse::new(reader.into_iter().peekable());
    stack.next(); // xml start tag
    let actual_tag_name = peek_at_name(&mut stack);
    match actual_tag_name {
        Ok(result) => {
            let res = IdologyIdResponseDeserializer::deserialize(&result, &mut stack);
            println!("{:#?}", res);
        }
        Err(_) => println!("error"),
    }
}
