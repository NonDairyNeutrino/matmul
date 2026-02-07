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

    ! create an identity matrix of the given dimension
    function id_mat(dim)
        implicit none
        integer, intent(in) :: dim
        real(f64)           :: id_mat(dim, dim)
        integer             :: i, j

        ! initialize with all zeros
        id_mat(:,:) = 0.0_f64
        ! replace diagonal with ones
        cols: do j = 1, dim
            rows: do i = 1, dim
                if (i .eq. j) then
                    id_mat(i, j) = 1.0_f64
                end if
            end do rows
        end do cols
    end function id_mat

end module matrix
