cmake_minimum_required(VERSION 3.8)

aux_source_directory(./ xtransport_test_src)

add_executable(xtransport_test ${xtransport_test_src})

add_dependencies(xtransport_test xtransport)

target_link_libraries(xtransport_test PRIVATE xcommon xtransport gtest gtest_main pthread )