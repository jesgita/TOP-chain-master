cmake_minimum_required(VERSION 3.8)

aux_source_directory(./ xcontract_common_test_src)

add_executable(xcontract_common_test ${xcontract_common_test_src})

add_dependencies(xcontract_common_test xcontract_common)

target_link_libraries(xcontract_common_test PRIVATE xcontract_common xdata gtest)