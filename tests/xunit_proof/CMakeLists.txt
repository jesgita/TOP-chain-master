cmake_minimum_required(VERSION 3.8)

aux_source_directory(./ xunit_proof_test_src)

add_executable(xunit_proof_test ${xunit_proof_test_src})

target_link_libraries(xunit_proof_test PRIVATE xvledger xblockstore xtxstore xblockmaker xverifier xcertauth xdata ssl crypto xxbase secp256k1 xconfig xloader xgenesis gtest pthread)
