// pallet_kitties
pub trait WeightInfo {
	fn create() -> Weight;
	fn breed() -> Weight;
	fn transfer() -> Weight;
	fn set_price() -> Weight;
	fn buy() -> Weight;
}
