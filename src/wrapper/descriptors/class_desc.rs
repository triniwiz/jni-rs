use strings::JNIString;

use objects::JClass;

use descriptors::Desc;

/// Specialization of the `Desc` type for use in class descriptors.
pub struct ClassDesc<'a, S: Into<JNIString>>(pub Desc<S, JClass<'a>>);

/// Trait representing anything that can be turned into a class descriptor.
/// Implementations are provided for `ClassDesc`, `Desc<S, JClass>`, `JClass`,
/// and anything that implements `Into<JNIString>`.
///
/// Implementors are better off implementing `Into<JNIString>` and letting the
/// provided implementation for it do the rest.
pub trait IntoClassDesc<'a> {
    type Desc: Into<JNIString>;

    fn into_desc(self) -> ClassDesc<'a, Self::Desc>;
}

impl<'a, S> IntoClassDesc<'a> for ClassDesc<'a, S>
    where S: Into<JNIString>
{
    type Desc = S;

    fn into_desc(self) -> ClassDesc<'a, S> {
        self
    }
}

impl<'a, S> IntoClassDesc<'a> for S
    where S: Into<JNIString>
{
    type Desc = S;

    fn into_desc(self) -> ClassDesc<'a, S> {
        ClassDesc(Desc::Descriptor(self))
    }
}

impl<'a> IntoClassDesc<'a> for JClass<'a> {
    type Desc = &'static str;

    fn into_desc(self) -> ClassDesc<'a, &'static str> {
        ClassDesc(Desc::Value(self))
    }
}

impl<'a, S> IntoClassDesc<'a> for Desc<S, JClass<'a>>
    where S: Into<JNIString>
{
    type Desc = S;

    fn into_desc(self) -> ClassDesc<'a, S> {
        ClassDesc(self)
    }
}