#add_compile_options(-static -Wall -Werror)
add_compile_options(-static)

aux_source_directory(./ xblockstore_mock_dir)
add_executable(xblockstore_mock ${xblockstore_mock_dir})

add_dependencies(xblockstore_mock xblockstore xmetrics)
include_directories(src/xtopcom/xblockstore/ tests/)
target_link_libraries(xblockstore_mock PRIVATE
    xblockstore
    xmetrics
)

if (XENABLE_TESTS)
    enable_testing()
endif()
