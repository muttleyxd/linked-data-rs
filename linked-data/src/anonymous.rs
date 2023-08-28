use iref::Iri;
use rdf_types::{IriVocabularyMut, Vocabulary};

use crate::{
	GraphVisitor, LexicalRepresentation, LinkedData, LinkedDataGraph, LinkedDataPredicateObjects,
	LinkedDataSubject, PredicateObjectsVisitor, RdfTerm, SubjectVisitor, Visitor,
};

pub struct AnonymousBinding<'a, T>(pub &'a Iri, pub &'a T);

impl<'a, T> AnonymousBinding<'a, T> {
	pub fn new(iri: &'a Iri, value: &'a T) -> Self {
		Self(iri, value)
	}
}

impl<'a, V: Vocabulary, I, T> LexicalRepresentation<V, I> for AnonymousBinding<'a, T> {
	fn lexical_representation(
		&self,
		_interpretation: &mut I,
		_vocabulary: &mut V,
	) -> Option<RdfTerm<V>> {
		None
	}
}

impl<'a, V: Vocabulary + IriVocabularyMut, I, T: LinkedDataPredicateObjects<V, I>>
	LinkedDataSubject<V, I> for AnonymousBinding<'a, T>
{
	fn visit_subject<S>(&self, mut serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<V, I>,
	{
		serializer.predicate(self.0, self.1)?;
		serializer.end()
	}
}

impl<'a, V: Vocabulary + IriVocabularyMut, I, T: LinkedDataPredicateObjects<V, I>>
	LinkedDataPredicateObjects<V, I> for AnonymousBinding<'a, T>
{
	fn visit_objects<S>(&self, mut serializer: S) -> Result<S::Ok, S::Error>
	where
		S: PredicateObjectsVisitor<V, I>,
	{
		serializer.object(self)?;
		serializer.end()
	}
}

impl<'a, V: Vocabulary + IriVocabularyMut, I, T: LinkedDataPredicateObjects<V, I>>
	LinkedDataGraph<V, I> for AnonymousBinding<'a, T>
{
	fn visit_graph<S>(&self, mut serializer: S) -> Result<S::Ok, S::Error>
	where
		S: GraphVisitor<V, I>,
	{
		serializer.subject(self)?;
		serializer.end()
	}
}

impl<'a, V: Vocabulary + IriVocabularyMut, I, T: LinkedDataPredicateObjects<V, I>> LinkedData<V, I>
	for AnonymousBinding<'a, T>
{
	fn visit<S>(&self, mut serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Visitor<V, I>,
	{
		serializer.default_graph(self)?;
		serializer.end()
	}
}
