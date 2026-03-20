program main
    use, intrinsic :: iso_fortran_env, only: f64=>real64
    use matrix, only: print_matrix, id_mat

    implicit none
    integer                :: dim
    real(f64), allocatable :: id(:,:)

    ! get user defined number of rows and columns
    print '(A)', "Enter an integer:"
    read (*, *) dim
    ! allocate matrix using user defined dimensions
    allocate(id(dim, dim))
    ! initialize to zeros
    id(:,:) = 0.0_f64
    ! print the matrix
    call print_matrix(id)
    id = id_mat(dim)
    call print_matrix(id)
    ! deallocate(matrix) ! allocatable arrays are automatically deallocated

end program main
