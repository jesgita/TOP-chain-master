cmake_minimum_required(VERSION 3.8)

aux_source_directory(./src xgrpc_mgr_src)
add_compile_options(-Wall)
add_compile_options(-Werror)
add_library(xgrpc_mgr ${xgrpc_mgr_src})

target_link_libraries(xgrpc_mgr PRIVATE
    xrpc xgrpcservice grpc grpc++_unsecure grpc_unsecure gpr cares address_sorting ssl crypto
)

if (BUILD_METRICS)
    #add_dependencies(xgrpc_mgr xmetrics)
    target_link_libraries(xgrpc_mgr PRIVATE xmetrics)
endif()
