cmake_minimum_required(VERSION 3.8)

add_compile_options(-Wpedantic)

aux_source_directory(./ xevm_runner_src)
aux_source_directory(proto/ xevm_runner_proto_src)

add_library(xevm_runner STATIC ${xevm_runner_src} ${xevm_runner_proto_src})

add_dependencies(xevm_runner xdata xcontract_common)

target_link_libraries(xevm_runner PRIVATE xdata xcontract_common protobuf)
