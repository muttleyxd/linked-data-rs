use iref::{Iri, IriBuf};
use rdf_types::{
	interpretation::{ReverseBlankIdInterpretation, ReverseIriInterpretation},
	BlankId, BlankIdBuf, Id, Interpretation, ReverseIdInterpretation, Vocabulary,
};

use crate::{FromLinkedDataError, LinkedDataGraph, LinkedDataPredicateObjects, LinkedDataResource};

/// Serialize a Linked-Data node.
pub trait LinkedDataSubject<I: Interpretation = (), V: Vocabulary = ()> {
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>;
}

impl<I: Interpretation, V: Vocabulary> LinkedDataSubject<I, V> for () {
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		serializer.end()
	}
}

impl<'a, I: Interpretation, V: Vocabulary, T: ?Sized + LinkedDataSubject<I, V>>
	LinkedDataSubject<I, V> for &'a T
{
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		T::visit_subject(self, serializer)
	}
}

impl<I: Interpretation, V: Vocabulary, T: ?Sized + LinkedDataSubject<I, V>> LinkedDataSubject<I, V>
	for Box<T>
{
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		T::visit_subject(self, serializer)
	}
}

impl<I: Interpretation, V: Vocabulary> LinkedDataSubject<I, V> for Iri {
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		serializer.end()
	}
}

impl<I: Interpretation, V: Vocabulary> LinkedDataSubject<I, V> for IriBuf {
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		serializer.end()
	}
}

impl<I: Interpretation, V: Vocabulary> LinkedDataSubject<I, V> for BlankId {
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		serializer.end()
	}
}

impl<I: Interpretation, V: Vocabulary> LinkedDataSubject<I, V> for BlankIdBuf {
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		serializer.end()
	}
}

impl<I: Interpretation, V: Vocabulary, T, B> LinkedDataSubject<I, V> for Id<T, B>
where
	T: LinkedDataSubject<I, V>,
	B: LinkedDataSubject<I, V>,
{
	fn visit_subject<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: SubjectVisitor<I, V>,
	{
		match self {
			Self::Iri(i) => i.visit_subject(serializer),
			Self::Blank(b) => b.visit_subject(serializer),
		}
	}
}

pub trait SubjectVisitor<I: Interpretation, V: Vocabulary> {
	type Ok;
	type Error;

	/// Visit a predicate of the graph.
	fn predicate<L, T>(&mut self, predicate: &L, objects: &T) -> Result<(), Self::Error>
	where
		L: ?Sized + LinkedDataResource<I, V>,
		T: ?Sized + LinkedDataPredicateObjects<I, V>;

	/// Visit a reverse predicate of the graph.
	fn reverse_predicate<L, T>(&mut self, predicate: &L, subjects: &T) -> Result<(), Self::Error>
	where
		L: ?Sized + LinkedDataResource<I, V>,
		T: ?Sized + LinkedDataPredicateObjects<I, V>;

	fn graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataGraph<I, V>;

	fn include<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataResource<I, V> + LinkedDataSubject<I, V>;

	fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl<'s, I: Interpretation, V: Vocabulary, S: SubjectVisitor<I, V>> SubjectVisitor<I, V>
	for &'s mut S
{
	type Ok = ();
	type Error = S::Error;

	fn predicate<L, T>(&mut self, predicate: &L, objects: &T) -> Result<(), Self::Error>
	where
		L: ?Sized + LinkedDataResource<I, V>,
		T: ?Sized + LinkedDataPredicateObjects<I, V>,
	{
		S::predicate(self, predicate, objects)
	}

	fn reverse_predicate<L, T>(&mut self, predicate: &L, subjects: &T) -> Result<(), Self::Error>
	where
		L: ?Sized + LinkedDataResource<I, V>,
		T: ?Sized + LinkedDataPredicateObjects<I, V>,
	{
		S::reverse_predicate(self, predicate, subjects)
	}

	fn graph<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataGraph<I, V>,
	{
		S::graph(self, value)
	}

	fn include<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + LinkedDataResource<I, V> + LinkedDataSubject<I, V>,
	{
		S::include(self, value)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

pub trait LinkedDataDeserializeSubject<I: Interpretation = (), V: Vocabulary = ()>: Sized {
	fn deserialize_subject<D>(
		vocabulary: &V,
		interpretation: &I,
		dataset: &D,
		graph: &D::Graph,
		resource: &I::Resource,
	) -> Result<Self, FromLinkedDataError>
	where
		D: grdf::Dataset<
			Subject = I::Resource,
			Predicate = I::Resource,
			Object = I::Resource,
			GraphLabel = I::Resource,
		>;
}

impl<I: Interpretation, V: Vocabulary> LinkedDataDeserializeSubject<I, V> for IriBuf
where
	I: ReverseIriInterpretation<Iri = V::Iri>,
{
	fn deserialize_subject<D>(
		vocabulary: &V,
		interpretation: &I,
		_dataset: &D,
		_graph: &D::Graph,
		resource: &I::Resource,
	) -> Result<Self, FromLinkedDataError>
	where
		D: grdf::Dataset<
			Subject = I::Resource,
			Predicate = I::Resource,
			Object = I::Resource,
			GraphLabel = I::Resource,
		>,
	{
		match interpretation.iris_of(resource).next() {
			Some(i) => {
				let iri = vocabulary.iri(i).unwrap();
				Ok(iri.to_owned())
			}
			None => Err(FromLinkedDataError::InvalidSubject),
		}
	}
}

impl<I: Interpretation, V: Vocabulary> LinkedDataDeserializeSubject<I, V> for BlankIdBuf
where
	I: ReverseBlankIdInterpretation<BlankId = V::BlankId>,
{
	fn deserialize_subject<D>(
		vocabulary: &V,
		interpretation: &I,
		_dataset: &D,
		_graph: &D::Graph,
		resource: &I::Resource,
	) -> Result<Self, FromLinkedDataError>
	where
		D: grdf::Dataset<
			Subject = I::Resource,
			Predicate = I::Resource,
			Object = I::Resource,
			GraphLabel = I::Resource,
		>,
	{
		match interpretation.blank_ids_of(resource).next() {
			Some(b) => {
				let blank_id = vocabulary.blank_id(b).unwrap();
				Ok(blank_id.to_owned())
			}
			None => Err(FromLinkedDataError::InvalidSubject),
		}
	}
}

impl<I: Interpretation, V: Vocabulary> LinkedDataDeserializeSubject<I, V> for Id
where
	I: ReverseIdInterpretation<Iri = V::Iri, BlankId = V::BlankId>,
{
	fn deserialize_subject<D>(
		vocabulary: &V,
		interpretation: &I,
		_dataset: &D,
		_graph: &D::Graph,
		resource: &I::Resource,
	) -> Result<Self, FromLinkedDataError>
	where
		D: grdf::Dataset<
			Subject = I::Resource,
			Predicate = I::Resource,
			Object = I::Resource,
			GraphLabel = I::Resource,
		>,
	{
		match interpretation.ids_of(resource).next() {
			Some(Id::Iri(i)) => {
				let iri = vocabulary.iri(i).unwrap();
				Ok(Id::Iri(iri.to_owned()))
			}
			Some(Id::Blank(b)) => {
				let blank_id = vocabulary.blank_id(b).unwrap();
				Ok(Id::Blank(blank_id.to_owned()))
			}
			None => Err(FromLinkedDataError::InvalidSubject),
		}
	}
}

impl<I: Interpretation, V: Vocabulary, T: LinkedDataDeserializeSubject<I, V>>
	LinkedDataDeserializeSubject<I, V> for Box<T>
{
	fn deserialize_subject<D>(
		vocabulary: &V,
		interpretation: &I,
		dataset: &D,
		graph: &D::Graph,
		resource: &I::Resource,
	) -> Result<Self, FromLinkedDataError>
	where
		D: grdf::Dataset<
			Subject = I::Resource,
			Predicate = I::Resource,
			Object = I::Resource,
			GraphLabel = I::Resource,
		>,
	{
		T::deserialize_subject(vocabulary, interpretation, dataset, graph, resource).map(Box::new)
	}
}
