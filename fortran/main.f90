program main
    use, intrinsic :: iso_fortran_env, only: f64=>real64
    use matrix, only: print_matrix

    implicit none
    integer(f64)           :: nrows, ncols
    real(f64), allocatable :: mat(:,:)

    ! get user defined number of rows and columns
    print *, "Enter two integers:"
    read (*, *) nrows, ncols
    ! allocate matrix using user defined dimensions
    allocate(mat(nrows, ncols))
    mat(:,:) = 0.0_f64
    mat(1, 1) = 1.0_f64
    mat(2, 2) = 1.0_f64

    ! print the matrix
    call print_matrix(mat)
    ! deallocate(matrix) ! allocatable arrays are automatically deallocated

end program main
