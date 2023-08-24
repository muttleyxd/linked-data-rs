use iref::Iri;
use rdf_types::{
    literal::Type, Interpretation, IriVocabularyMut, LiteralVocabularyMut, Term, Vocabulary,
};

use crate::{
    LexicalRepresentation, PredicateSerializer, RdfLiteral, RdfTerm, SerializePredicate,
    SerializeSubject,
};

macro_rules! datatype {
    ($($ty:ident : $iri:literal),*) => {
        $(
            impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation> LexicalRepresentation<V, I> for $ty
            where
                V::Value: From<String>,
                V::Type: From<Type<V::Iri, V::LanguageTag>>,
            {
                fn lexical_representation(
                    &self,
                    _interpretation: &mut I,
                    vocabulary: &mut V
                ) -> Option<RdfTerm<V>> {
                    let ty = vocabulary.insert(Iri::new($iri).unwrap());
                    Some(Term::Literal(RdfLiteral::Any(
                        self.to_string(),
                        Type::Any(ty).into(),
                    )))
                }
            }

            impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation> SerializeSubject<V, I> for $ty
            where
                V::Value: From<String>,
                V::Type: From<Type<V::Iri, V::LanguageTag>>,
            {
                fn serialize_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: crate::SubjectSerializer<V, I>
                {
                    serializer.end()
                }
            }

            impl<V: Vocabulary + IriVocabularyMut + LiteralVocabularyMut, I: Interpretation> SerializePredicate<V, I> for $ty
            where
                V::Value: From<String>,
                V::Type: From<Type<V::Iri, V::LanguageTag>>,
            {
                fn serialize_predicate<S>(&self, mut serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: PredicateSerializer<V, I>,
                {
                    serializer.insert(self)?;
                    serializer.end()
                }
            }
        )*
    };
}

datatype! {
    u8: "http://www.w3.org/2001/XMLSchema#unsignedByte",
    u16: "http://www.w3.org/2001/XMLSchema#unsignedShort",
    u32: "http://www.w3.org/2001/XMLSchema#unsignedInt",
    u64: "http://www.w3.org/2001/XMLSchema#unsignedLong",
    i8: "http://www.w3.org/2001/XMLSchema#byte",
    i16: "http://www.w3.org/2001/XMLSchema#short",
    i32: "http://www.w3.org/2001/XMLSchema#int",
    i64: "http://www.w3.org/2001/XMLSchema#long",
    String: "http://www.w3.org/2001/XMLSchema#string"
}
