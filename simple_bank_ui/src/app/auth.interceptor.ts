import { Injectable } from '@angular/core';
import {
  HttpEvent, HttpInterceptor, HttpHandler, HttpRequest, HttpErrorResponse
} from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { Router } from '@angular/router';
import { StorageService } from './_services/storage.service';

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

  constructor(private storageService: StorageService, private router: Router) { }

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    return next.handle(req).pipe(
      catchError((error: HttpErrorResponse) => {
        if (error.status === 401) { 
          // If the server returns a 401 status (unauthorized), it means the session has expired
          // Perform your logout operation here
          // Navigate the user to the login page
          this.storageService.clean();
          this.router.navigate(['/login']).then(() => {
            window.location.reload();
          });
        }
        return throwError(error);
      })
    );
  }
}
