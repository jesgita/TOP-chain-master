#add_compile_options(-static -Wall -Werror)
add_compile_options(-static)

include_directories(.)
include_directories(./xcontract)
include_directories(./xerror)
include_directories(./src)

aux_source_directory(./src src_dir)
add_library(xtxexecutor STATIC ${src_dir})

target_link_libraries(xtxexecutor PRIVATE
    xblockstore
    xstore
    xstatectx
    xgasfee
    xverifier
    xvm
    xchain_upgrade
    xdata
    xxbase
)

if (BUILD_EVM)
    target_link_libraries(xtxexecutor PRIVATE xevm)
endif()

if (BUILD_METRICS)
    # add_dependencies(xtxexecutor xmetrics)
    target_link_libraries(xtxexecutor PRIVATE xmetrics)
endif()
