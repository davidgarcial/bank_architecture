import { Component } from '@angular/core';
import { AccountService } from '../_services/account.service';

@Component({
  selector: 'app-update-account',
  templateUrl: './update-account.component.html',
  styleUrls: ['./update-account.component.scss'],
})
export class UpdateAccountComponent {
  accountId: string = '';
  accountName: string = '';
  balance: number = 0;

  constructor(private accountService: AccountService) {}

  onSubmit() {
    this.accountService
      .updateAccount(this.accountId, this.accountName, this.balance)
      .subscribe(
        (response) => {
          console.log('Account updated successfully', response);
        },
        (error) => {
          console.error('Error updating account', error);
        }
      );
  }
}
