add_compile_options(-Wpedantic)

aux_source_directory(./src src_dir)
add_library(xevm_contract_runtime STATIC ${src_dir})

add_dependencies(xevm_contract_runtime xevm_engine)
get_target_property(EVM_ENGINE_DIR xevm_engine LOCATION)
target_link_libraries(xevm_contract_runtime PRIVATE ${EVM_ENGINE_DIR}/libxevm_engine.a)

add_dependencies(xevm_contract_runtime xevm_runner xcontract_runtime xcontract_common xevm_common xbasic xcommon)

target_link_libraries(xevm_contract_runtime PRIVATE xevm_runner xcontract_runtime xcontract_common xevm_common xbasic xcommon ethash keccak)
