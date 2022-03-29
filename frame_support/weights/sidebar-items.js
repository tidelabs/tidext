initSidebarItems({"enum":[["DispatchClass","A generalized group of dispatch types."],["Pays","Explicit enum to denote if a transaction pays fee or not."]],"fn":[["extract_actual_weight","Extract the actual weight from a dispatch result if any or fall back to the default weight."]],"mod":[["constants","These constants are specific to FRAME, and the current implementation of its various components. For example: FRAME System, FRAME Executive, our FRAME support libraries, etc…"]],"struct":[["DispatchInfo","A bundle of static information collected from the `#[weight = $x]` attributes."],["FunctionOf","A struct to represent a weight which is a function of the input arguments. The given items have the following types:"],["IdentityFee","Implementor of `WeightToFeePolynomial` that maps one unit of weight to one unit of fee."],["PerDispatchClass","A struct holding value for each `DispatchClass`."],["PostDispatchInfo","Weight information that is only available post dispatch. NOTE: This can only be used to reduce the weight or fee, not increase it."],["RuntimeDbWeight","The weight of database operations that the runtime can invoke."],["WeightToFeeCoefficient","One coefficient and its position in the `WeightToFeePolynomial`."]],"trait":[["ClassifyDispatch","Means of classifying a dispatchable function."],["GetDispatchInfo","A `Dispatchable` function (aka transaction) that can carry some static information along with it, using the `#[weight]` attribute."],["OneOrMany","A trait that represents one or many values of given type."],["PaysFee","Indicates if dispatch function should pay fees or not. If set to `Pays::No`, the block resource limits are applied, yet no fee is deducted."],["WeighData","Means of weighing some particular kind of data (`T`)."],["WeightToFeePolynomial","A trait that describes the weight to fee calculation as polynomial."],["WithPostDispatchInfo","Allows easy conversion from `DispatchError` to `DispatchErrorWithPostInfo` for dispatchables that want to return a custom a posterior weight on error."]],"type":[["TransactionPriority","Re-export priority as type Priority for a transaction. Additive. Higher is better."],["Weight","Numeric range of a transaction weight."],["WeightToFeeCoefficients","A list of coefficients that represent one polynomial."]]});