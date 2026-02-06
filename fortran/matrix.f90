module matrix
    use, intrinsic :: iso_fortran_env, only: f64=>real64
    implicit none

    contains

    ! print the elements of a matrix to stdout
    subroutine print_matrix(matrix)
        implicit none
        ! the dimensions of the matrix are known at call-time
        real(f64), intent(in) :: matrix(:,:)
        integer(f64) :: i, j

        print "('Printing ', i1, ' rows and ', i1, ' cols.')", size(matrix, 1), size(matrix, 2)

        cols: do j = 1, size(matrix, 2)
            print *, matrix(:, j)
        end do cols
    end subroutine print_matrix

end module matrix
