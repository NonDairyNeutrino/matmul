program main
    use, intrinsic :: iso_fortran_env, only: real32, real64
    use matrix, only: print_matrix
    implicit none
    integer(real64) :: nrows, ncols
    real(real64), allocatable :: mat(:,:)

    ! get user defined number of rows and columns
    print *, 'Enter two integers:'
    read (*, *) nrows, ncols
    ! allocate matrix using user defined dimensions
    allocate(mat(nrows, ncols))
    mat(:,:) = 0.0

    ! print the matrix
    call print_matrix(mat)
    ! deallocate(matrix) ! allocatable arrays are automatically deallocated

end program main
